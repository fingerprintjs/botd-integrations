mod config;
mod utils;
mod injector;
mod detector;
mod botd;
mod edge;
mod endpoint;

use fastly::{Error, mime, Request, Response};
use fastly::http::header;
use header::HOST;
use log::LevelFilter::Debug;
use mime::TEXT_HTML_UTF_8;

use botd::BotDetector;
use edge::EdgeDetect;

use crate::config::{Config, APP_BACKEND_NAME};
use crate::detector::{Detect, ERROR};
use crate::injector::inject_script;
use crate::utils::{get_ip, is_static_requested};

const SET_COOKIE_HEADER:            &str = "set-cookie";
pub const COOKIE_NAME:              &str = "botd-request-id=";
pub const REQUEST_ID_HEADER:        &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER:    &str = "botd-request-status";
pub const ERROR_DESCRIPTION:        &str = "botd-error-description";

pub fn set_error(req: &mut Request, desc: String, request_id: Option<String>) {
    req.set_header(REQUEST_ID_HEADER, request_id.unwrap_or_else(|| String::from("")));
    req.set_header(REQUEST_STATUS_HEADER, ERROR);
    req.set_header(ERROR_DESCRIPTION, desc);
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // TODO: get rid of it
    req.set_pass(true);

    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            set_error(&mut req, e.to_string(), None);
            return Ok(req.send(APP_BACKEND_NAME)?)
        }
    };

    log_fastly::init_simple(config.log_endpoint_name.to_owned(), Debug);
    log::debug!("[main] Request received from: {}, url: {}", get_ip(&req), req.get_url_str());

    // Set HOST header for CORS policy
    if let Some(h) = config.app_host {
        log::debug!("[main] Application host: {}", h);
        req.set_header(HOST, h);
    }

    return match req.get_path() {
        "/" => {
            log::debug!("[main] Initial request, starting edge detect");

            let mut request = req.clone_with_body();
            let edge = EdgeDetect::make(&mut request, &config)?;
            let response = request.send(APP_BACKEND_NAME)?;

            log::debug!("[main] Insert botd script");

            let new_body = inject_script(&response.into_body_str(), &config);
            let cookie_value = format!("{}{}", COOKIE_NAME, edge.get_request_id());

            Ok(response
                .with_content_type(TEXT_HTML_UTF_8)
                .with_header(SET_COOKIE_HEADER, cookie_value)
                .with_body(new_body))
        }
        _ => {
            if is_static_requested(&req) {
                if req.get_path().ends_with(".ico") {
                    log::debug!("[main] favicon request, starting light detect");
                    EdgeDetect::make(&mut req, &config)?;
                    return Ok(req.send(APP_BACKEND_NAME)?);
                }
                log::debug!("[main] path: {}, static requested => skipped bot detection", req.get_path().to_owned());
                return Ok(req.send(APP_BACKEND_NAME)?);
            }

            log::debug!("[main] path: {}, not static => do bot detection", req.get_path().to_owned());

            BotDetector::make(&mut req, &config)?;
            Ok(req.send(APP_BACKEND_NAME)?)
        }
    }
}
