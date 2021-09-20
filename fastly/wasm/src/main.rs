mod request_id;
mod injector;
mod detector;
mod config;
mod error;
mod utils;
mod botd;
mod edge;

use fastly::{Error, Request, Response};
use fastly::http::header;
use header::HOST;
use header::SET_COOKIE;
use botd::BotDetector;
use edge::EdgeDetect;
use BotdError::SendError;
use crate::config::{Config, APP_BACKEND_NAME, BOTD_BACKEND_NAME, CDN_BACKEND_NAME};
use crate::utils::{is_static_requested, make_cookie, is_favicon_requested, get_ip};
use crate::detector::Detect;
use crate::injector::inject_script;
use crate::request_id::RequestId;
use crate::error::{handle_error, BotdError};

const PATH_HASH: &str = "2f70092c";

pub const REQUEST_ID_HEADER_COOKIE: &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER: &str = "botd-request-status";
pub const ERROR_DESCRIPTION_HEADER: &str = "botd-error-description";

fn init_req_handler(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Initial request, starting edge detect");
    let mut req_with_botd_headers = req.clone_with_body();
    let edge = match EdgeDetect::make(&mut req_with_botd_headers, config) {
        Ok(d) => d,
        Err(e) => return handle_error(req, e, Some(config), true)
    };
    log::debug!("[main] Edge detect request id: {}", edge.req_id);
    let resp = req_with_botd_headers.send(APP_BACKEND_NAME)?;
    let resp_without_body = resp.clone_without_body();
    log::debug!("[main] Insert botd script");
    let body = resp.into_body_str();
    let new_body = match inject_script(&body, config) {
        Ok(b) => b,
        Err(e) => return handle_error(req, e, Some(config), true)
    };
    let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), edge.req_id);
    log::debug!("[main] Set cookie to initial response: {}", cookie);
    Ok(resp_without_body
        .with_header(SET_COOKIE, cookie)
        .with_body(new_body))
}

fn detect_req_handler(req: Request, config: &Config) -> Result<Response, Error> {
    let req_clone = req.clone_without_body();
    let mut botd_resp = match req
        .with_path("/api/v1/detect")
        .send(BOTD_BACKEND_NAME) {
        Ok(r) => r,
        Err(e) => return handle_error(req_clone, SendError(e.root_cause().to_string()), Some(config), false)
    };
    let botd_resp_clone = botd_resp.clone_with_body();
    let req_id = RequestId::from_resp_body(botd_resp_clone).unwrap_or_default();
    let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), req_id);
    log::debug!("[main] Set cookie to detect response: {}", cookie);
    Ok(botd_resp.with_header(SET_COOKIE, cookie))
}

fn dist_req_handler(req: Request, config: &Config) -> Result<Response, Error> {
    let path_req = req.clone_without_body();
    // let cut_version_path = |p: &str| &p[format!("/{}/detect", PATH_HASH).len()..p.len()];

    let path = path_req.get_path();
    let cdn_path = &path[format!("/{}/dist", PATH_HASH).len()..path.len()];
    log::debug!("[dist] CDN path: {}", cdn_path);
    let err_req = req.clone_without_body();
    match req
        .with_path(cdn_path)
        .with_pass(false)
        .send(CDN_BACKEND_NAME) {
        Ok(r) => Ok(r),
        Err(e) => handle_error(err_req, SendError(e.root_cause().to_string()), Some(config), false)
    }
}

fn favicon_req_handler(req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Favicon request => starting edge detect");
    let mut req_with_edge_headers = req.clone_without_body();
    return match EdgeDetect::make(&mut req_with_edge_headers, config) {
        Ok(d) => {
            let response = req.send(APP_BACKEND_NAME)?;
            let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), d.req_id);
            log::debug!("[main] Set cookie to favicon response: {}", cookie);
            Ok(response.with_header(SET_COOKIE, cookie))
        }
        Err(e) => handle_error(req, e, Some(config), true)
    };
}

fn static_req_handler(req: Request) -> Result<Response, Error> {
    log::debug!("[main] Static request => skipped bot detection");
    Ok(req.send(APP_BACKEND_NAME)?)
}

fn non_static_req_handler(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Not static request => do bot detection");
    let err_req = req.clone_without_body();
    match BotDetector::make(&mut req, config) {
        Ok(_) => Ok(req.send(APP_BACKEND_NAME)?),
        Err(e) => handle_error(err_req, e, Some(config), true)
    }
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // TODO: get rid of it
    req.set_pass(true);

    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => return handle_error(req, e, None, true)
    };

    log::debug!("[main] New request {}, ip address: {}", req.get_url_str(), get_ip(&req));

    // Set HOST header for CORS policy
    if let Some(h) = config.app_host.to_owned() {
        log::debug!("[main] Set header host: {}", h);
        req.set_header(HOST, h);
    }

    return match req.get_path() {
        "/" => init_req_handler(req, &config),
        p if p == format!("/{}/detect", PATH_HASH) => detect_req_handler(req, &config),
        p if p.starts_with(&format!("/{}/dist", PATH_HASH)) => dist_req_handler(req, &config),
        _ if is_favicon_requested(&req) => favicon_req_handler(req, &config),
        _ if is_static_requested(&req) => static_req_handler(req),
        _ => non_static_req_handler(req, &config)
    };
}