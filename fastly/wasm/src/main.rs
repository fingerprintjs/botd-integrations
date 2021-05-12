//! Default Compute@Edge template program.

mod injector;
mod web_utils;
mod result_item;
mod bot_detector;
mod constants;
mod config;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use constants::*;
use injector::add_bot_detection_script;
use crate::config::read_config;
use crate::bot_detector::handle_request_with_bot_detect;
use crate::web_utils::{extract_header_value, is_static_requested};

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    log_fastly::init_simple(LOGGER, log::LevelFilter::Debug);
    log::debug!("Request received from: {}", req.get_client_ip_addr().unwrap().to_string().as_str());

    let config_result = read_config();
    if config_result.is_err() {
        return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_body_str("Cannot read Fastly configuration\n"))
    }
    let config = config_result.unwrap();

    // Set HOST header for CORS policy
    let mut app_backend_host = config.app_backend_url.to_string();
    if app_backend_host.starts_with("http://") {
        app_backend_host = (&app_backend_host["http://".len()..]).parse()?;
    } else if app_backend_host.starts_with("https://") {
        app_backend_host = (&app_backend_host["https://".len()..]).parse()?;
    }
    req.set_header(header::HOST, app_backend_host.to_owned());

    // Filter request methods...
    match req.get_method() {
        // Allow GET, POST, HEAD requests.
        &Method::GET | &Method::HEAD | &Method::POST => (),

        &Method::OPTIONS => {
            req.set_ttl(86400);
            return Ok(req.send(APP_BACKEND)?);
        }

        // Accept PURGE requests; it does not matter to which backend they are sent.
        m if m == "PURGE" => return Ok(req.send(APP_BACKEND)?),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, POST, OPTIONS")
                .with_body_str("This method is not allowed\n"))
        }
    };

    // Pattern match on the path.
    return match req.get_path() {
        "/" => {
            req.set_pass(true); // TODO: get rid of it

            let request = Request::get(config.app_backend_url.to_owned());
            let response = request.send(APP_BACKEND).unwrap();
            let html_with_script = add_bot_detection_script(Box::from(response.into_body_str()), &config);

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(html_with_script))
        }
        _ => {
            req.set_pass(true); // TODO: get rid of it

            if is_static_requested(&req) {
                return Ok(req.send(APP_BACKEND).unwrap());
            }

            return Ok(handle_request_with_bot_detect(req, &config))
        }
    }
}
