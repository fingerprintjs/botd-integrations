mod config;
mod utils;
mod injector;
mod detector;
mod botd;
mod edge;
mod endpoint;
mod error;

use fastly::{Error, Request, Response};
use fastly::http::header;
use header::HOST;
use header::SET_COOKIE;
use log::LevelFilter::Debug;
use botd::BotDetector;
use edge::EdgeDetect;
use crate::config::{Config, APP_BACKEND_NAME, BOTD_BACKEND_NAME, CDN_BACKEND_NAME};
use crate::detector::{Detect, ERROR};
use crate::injector::inject_script;
use crate::utils::{is_static_requested, make_cookie, is_favicon_requested};
use crate::endpoint::BotdEndpoint;

const PATH_HASH: &str = "2f70092c";

pub const REQUEST_ID_HEADER_COOKIE: &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER: &str = "botd-request-status";
pub const ERROR_DESCRIPTION_HEADER: &str = "botd-error-description";

fn send_error(req: Request, desc: String, request_id: Option<String>) -> Result<Response, Error> {
    log::error!("[error] {}", desc);
    Ok(req.with_header(REQUEST_ID_HEADER_COOKIE, request_id.unwrap_or_else(|| String::from("")))
        .with_header(REQUEST_STATUS_HEADER, ERROR)
        .with_header(ERROR_DESCRIPTION_HEADER, desc).send(APP_BACKEND_NAME)?)
}

fn initial_request(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Initial request, starting edge detect");
    let mut request = req.clone_with_body();
    let edge = match EdgeDetect::make(&mut request, config) {
        Ok(d) => d,
        Err(e) => return send_error(req, e.to_string(), None)
    };
    log::debug!("[main] Edge detect request id: {}", edge.request_id);
    let response = request.send(APP_BACKEND_NAME)?;
    let resp_clone = response.clone_without_body();
    log::debug!("[main] Insert botd script");
    let body = response.into_body_str();
    let new_body = match inject_script(&body, config) {
        Ok(b) => b,
        Err(e) => return send_error(req, e.to_string(), Some(edge.request_id))
    };
    let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), edge.request_id);
    log::debug!("[main] Set cookie to initial response: {}", cookie);
    Ok(resp_clone
        .with_header(SET_COOKIE, cookie)
        .with_body(new_body))
}

fn detect_request(req: Request) -> Result<Response, Error> {
    let endpoint = BotdEndpoint::new("/detect");
    let mut response = req
        .with_path(endpoint.path.as_str())
        .send(BOTD_BACKEND_NAME)?;
    let resp_clone = response.clone_with_body();
    let body = response.into_body_str();
    let request_id = match BotDetector::extract_request_id(body.as_str()) {
        Some(r) => r,
        _ => {
            log::error!("[error] Can't extract request id from body.");
            String::from("")
        }
    };
    let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), request_id);
    log::debug!("[main] Set cookie to detect response: {}", cookie);
    Ok(resp_clone.with_header(SET_COOKIE, cookie))
}

fn dist_request(req: Request) -> Result<Response, Error> {
    Ok(req
        .with_path("/")
        .with_pass(false)
        .send(CDN_BACKEND_NAME)?)
}

fn favicon_request(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Favicon request => starting edge detect");
    return match EdgeDetect::make(&mut req, config) {
        Ok(d) => {
            let response = req.send(APP_BACKEND_NAME)?;
            let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), d.request_id);
            log::debug!("[main] Set cookie to favicon response: {}", cookie);
            Ok(response.with_header(SET_COOKIE, cookie))
        }
        Err(e) => send_error(req, e.to_string(), None)
    };
}

fn static_request(req: Request) -> Result<Response, Error> {
    log::debug!("[main] Static request => skipped bot detection");
    Ok(req.send(APP_BACKEND_NAME)?)
}

fn non_static_request(mut req: Request, config: &Config) -> Result<Response, Error> {
    log::debug!("[main] Not static request => do bot detection");
    match BotDetector::make(&mut req, config) {
        Ok(d) => {
            Ok(req.send(APP_BACKEND_NAME)?)
            // let response = req.send(APP_BACKEND_NAME)?;
            // let cookie = make_cookie(String::from(REQUEST_ID_HEADER_COOKIE), d.request_id);
            // log::debug!("[main] Set cookie to non-static response: {}", cookie);
            // Ok(response.with_header(SET_COOKIE, cookie))
        },
        Err(e) => send_error(req, e.to_string(), None)
    }
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // TODO: get rid of it
    req.set_pass(true);
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => return send_error(req, e.to_string(), None)
    };
    log_fastly::init_simple(config.log_endpoint_name.to_owned(), Debug);
    let ip = match req.get_client_ip_addr() {
        Some(t) => t.to_string(),
        _ => String::from("0.0.0.0")
    };
    log::debug!("[main] New request received from: {}, url: {}", ip, req.get_url_str());
    // Set HOST header for CORS policy
    if let Some(h) = config.app_host.to_owned() {
        log::debug!("[main] Host header replacement to application host: {}", h);
        req.set_header(HOST, h);
    }

    let detect_path = format!("/{}/detect", PATH_HASH);
    let dist_path = format!("/{}/dist", PATH_HASH);

    return match req.get_path() {
        "/" => initial_request(req, &config),
        path if path == detect_path => detect_request(req),
        path if path.starts_with(&dist_path) => dist_request(req),
        _ if is_favicon_requested(&req) => favicon_request(req, &config),
        _ if !is_static_requested(&req) => non_static_request(req, &config),
        _ => static_request(req)
    };
}