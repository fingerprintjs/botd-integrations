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
use log::LevelFilter::Debug;
use botd::BotDetector;
use edge::EdgeDetect;
use crate::config::{Config, APP_BACKEND_NAME};
use crate::detector::{Detect, ERROR};
use crate::injector::inject_script;
use crate::utils::{is_static_requested};

const SET_COOKIE_HEADER:                &str = "set-cookie";
pub const REQUEST_ID_HEADER_COOKIE:     &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER:        &str = "botd-request-status";
pub const ERROR_DESCRIPTION_HEADER:     &str = "botd-error-description";

fn send_error(req: Request, desc: String, request_id: Option<String>) -> Result<Response, Error> {
    log::error!("ERROR: {}", desc);
    Ok(req.with_header(REQUEST_ID_HEADER_COOKIE, request_id.unwrap_or_else(|| String::from("")))
        .with_header(REQUEST_STATUS_HEADER, ERROR)
        .with_header(ERROR_DESCRIPTION_HEADER, desc).send(APP_BACKEND_NAME)?)
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
        _ => "0.0.0.0".to_string()
    };
    log::debug!("[main] Request received from: {}, url: {}", ip, req.get_url_str());
    // Set HOST header for CORS policy
    if let Some(h) = config.app_host.to_owned() {
        log::debug!("[main] Application host: {}", h);
        req.set_header(HOST, h);
    }

    return match req.get_path() {
        "/" => {
            log::debug!("[main] Initial request, starting edge detect");
            let mut request = req.clone_with_body();
            let edge = match EdgeDetect::make(&mut request, &config) {
                Ok(d) => d,
                Err(e) => return send_error(req, e.to_string(), None)
            };
            let response = request.send(APP_BACKEND_NAME)?;
            let resp_clone = response.clone_without_body();
            log::debug!("[main] Insert botd script");
            let body = response.into_body_str();
            let new_body = match inject_script(&body, &config){
                Ok(b) => b,
                Err(e) => return send_error(req, e.to_string(), Some(edge.request_id))
            };
            let cookie_value = format!("{}={}", REQUEST_ID_HEADER_COOKIE, edge.request_id);
            Ok(resp_clone
                .with_header(SET_COOKIE_HEADER, cookie_value)
                .with_body(new_body))
        }
        _ => {
            if is_static_requested(&req) {
                if req.get_path().ends_with(".ico") {
                    log::debug!("[main] favicon request, starting light detect");
                    return match EdgeDetect::make(&mut req, &config) {
                        Ok(d) => {
                            let cookie_value = format!("{}={}", REQUEST_ID_HEADER_COOKIE, d.request_id);
                            Ok(req
                                .with_header(SET_COOKIE_HEADER, cookie_value)
                                .send(APP_BACKEND_NAME)?)
                        },
                        Err(e) => { send_error(req, e.to_string(), None) }
                    };
                }
                log::debug!("[main] path: {}, static requested => skipped bot detection", req.get_path().to_owned());
                return Ok(req.send(APP_BACKEND_NAME)?);
            }
            log::debug!("[main] path: {}, not static => do bot detection", req.get_path().to_owned());
            return match BotDetector::make(&mut req, &config) {
                Ok(_) => Ok(req.send(APP_BACKEND_NAME)?) ,
                Err(e) => send_error(req, e.to_string(), None)
            };
        }
    }
}
