mod request_id;
mod injector;
mod detector;
mod config;
mod error;
mod utils;
mod botd;
mod edge;

use std::panic;
use fastly::{Error, Request, Response};
use fastly::http::header::{ACCEPT_ENCODING, SET_COOKIE};
use botd::BotDetector;
use edge::EdgeDetect;
use BotdError::SendError;
use crate::config::{Config, APP_BACKEND_NAME, BOTD_BACKEND_NAME, CDN_BACKEND_NAME};
use crate::utils::{is_static_requested, make_cookie, is_favicon_requested, get_e_tld_plus_one};
use crate::detector::Detect;
use crate::injector::inject_script;
use crate::request_id::RequestId;
use crate::error::{handle_error, BotdError, panic_hook};

const PATH_HASH: &str = "2f70092c";

pub const REQUEST_ID_HEADER_COOKIE: &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER: &str = "botd-request-status";
pub const ERROR_DESCRIPTION_HEADER: &str = "botd-error-description";
pub const CLIENT_IP_HEADER: &str = "botd-client-ip";

fn init_req_handler(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::info!("[main] Initial request, starting edge detect");
    let domain = get_e_tld_plus_one(&req);
    let mut req_with_botd_headers = req.clone_with_body();
    req_with_botd_headers.remove_header(ACCEPT_ENCODING);
    let req_id = match EdgeDetect::make(&mut req_with_botd_headers, config) {
        Ok(d) => d.req_id,
        Err(e) => return handle_error(req, e, Some(config), true)
    };
    log::debug!("[main] Edge detect request id: {}", req_id);
    let beresp = req_with_botd_headers.send(APP_BACKEND_NAME)?;
    let beresp_clone = beresp.clone_without_body();
    log::debug!("[main] Insert botd script");
    let body = beresp.into_body_str();
    let new_body = match inject_script(&body, config) {
        Ok(b) => b,
        Err(e) => return handle_error(req, e, Some(config), true)
    };
    let cookie = make_cookie(REQUEST_ID_HEADER_COOKIE, req_id, domain);
    log::debug!("[main] Set cookie to initial response: {}", cookie);
    Ok(beresp_clone
        .with_header(SET_COOKIE, cookie)
        .with_body(new_body))
}

fn detect_req_handler(req: Request, config: &Config) -> Result<Response, Error> {
    log::info!("[main] Detect request => redirecting to Botd");
    let domain = get_e_tld_plus_one(&req);
    let err_req = req.clone_without_body();
    let mut botd_resp = match req
        .with_path("/api/v1/detect")
        .with_header(CLIENT_IP_HEADER, config.ip.to_owned())
        .send(BOTD_BACKEND_NAME) {
        Ok(r) => r,
        Err(e) => return handle_error(err_req, SendError(e), Some(config), false)
    };
    let botd_resp_clone = botd_resp.clone_with_body();
    let req_id = RequestId::from_resp_body(botd_resp_clone).unwrap_or_default();
    let cookie = make_cookie(REQUEST_ID_HEADER_COOKIE, req_id, domain);
    log::debug!("[main] Set cookie to detect response: {}", cookie);
    Ok(botd_resp.with_header(SET_COOKIE, cookie))
}

fn dist_req_handler(req: Request, config: &Config, cdn_path: &str) -> Result<Response, Error> {
    log::info!("[main] Script request => redirecting to CDN");
    let err_req = req.clone_without_body();
    match req
        .with_path(cdn_path)
        // .with_pass(false)
        .send(CDN_BACKEND_NAME) {
        Ok(r) => Ok(r),
        Err(e) => handle_error(err_req, SendError(e), Some(config), false)
    }
}

fn favicon_req_handler(req: Request, config: &Config) -> Result<Response, Error> {
    log::info!("[main] Favicon request => starting edge detect");
    let mut edge_req = req.clone_without_body();
    match EdgeDetect::make(&mut edge_req, config) {
        // TODO: Fix edge detect cookie race
        Ok(_) => Ok(req.send(APP_BACKEND_NAME)?),
        Err(e) => handle_error(req, e, Some(config), true)
    }
}

fn static_req_handler(req: Request) -> Result<Response, Error> {
    log::info!("[main] Static request => skipped bot detection");
    Ok(req.send(APP_BACKEND_NAME)?)
}

fn non_static_req_handler(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::info!("[main] Not static request => do bot detection");
    match BotDetector::make(&mut req, config) {
        Ok(_) => Ok(req.send(APP_BACKEND_NAME)?),
        Err(e) => handle_error(req, e, Some(config), true)
    }
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    panic::set_hook(panic_hook());
    req.set_pass(true);

    let config = match Config::new(&req) {
        Ok(c) => c,
        Err(e) => return handle_error(req, e, None, true)
    };

    log::info!("[main] New request {}", req.get_url_str());
    log::debug!("[main] IP address: {}, headers: {:?}", config.ip, req.get_header_names_str());

    let dist_path = format!("/{}/dist", PATH_HASH);

    return match req.get_path() {
        "/" => init_req_handler(req, &config),
        p if p == format!("/{}/detect", PATH_HASH) => detect_req_handler(req, &config),
        p if p.starts_with(dist_path.as_str()) => {
            let path = p.to_owned();
            let cdn_path = &path[dist_path.len()..];
            log::debug!("[main] CDN path: {}", cdn_path);
            dist_req_handler(req, &config, cdn_path)
        },
        _ if is_favicon_requested(&req) => favicon_req_handler(req, &config),
        _ if is_static_requested(&req) => static_req_handler(req),
        _ => non_static_req_handler(req, &config)
    };
}