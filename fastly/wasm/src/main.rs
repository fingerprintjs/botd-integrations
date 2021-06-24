mod injector;
mod web_utils;
mod result_item;
mod bot_detector;
mod constants;
mod config;
mod light_detector;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use constants::*;
use injector::add_bot_detection_script;
use config::read_config;
use bot_detector::handle_request_with_bot_detect;
use web_utils::{is_static_requested};
use light_detector::{make_light_detect, set_light_headers};
use crate::web_utils::get_host_from_url;

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {

    let config_result = read_config();
    if config_result.is_err() {
        return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_body_str("Cannot read Fastly configuration\n"))
    }
    let config = config_result.unwrap();

    log_fastly::init_simple(config.env.to_owned(), log::LevelFilter::Debug);

    let ip = req.get_client_ip_addr().unwrap().to_string();
    log::debug!("[main] request received from: {}, url: {}", ip, req.get_url_str());

    // Set HOST header for CORS policy
    let backend_host_op = get_host_from_url(config.app_backend_url.to_owned());
    if backend_host_op.is_none() {
        log::error!("[main] wrong app backend url in config app_backend_url: {}", config.app_backend_url);
        return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_body_str(&format!("Wrong app backend url in Fastly configuration: {} \n", config.app_backend_url)))
    }
    let host = backend_host_op.unwrap();

    req.set_header(header::HOST, host.to_owned());
    log::debug!("[main] app backend host: {}", host);

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
            log::error!("[main] method is not allowed: {}", req.get_method());
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, POST, OPTIONS")
                .with_body_str("This method is not allowed\n"))
        }
    };

    match req.get_path() {
        "/" => {
            log::debug!("[main] initial request, starting light detect");
            req.set_pass(true); // TODO: get rid of it

            let light_result = make_light_detect(&req, &config);
            let id = light_result.request_id.clone();
            let mut request = Request::get(config.app_backend_url.to_owned());
            request = set_light_headers(request, light_result);

            let response = request.send(APP_BACKEND).unwrap();
            log::debug!("[main] inserting bot detection script");
            let html_with_script = add_bot_detection_script(Box::from(response.into_body_str()), &config);
            let cookie_value = format!("{}{}", COOKIE_NAME, id);

            return Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_header(SET_COOKIE_HEADER, cookie_value)
                .with_body(html_with_script))
        }
        _ => {
            req.set_pass(true); // TODO: get rid of it

            if is_static_requested(&req) {
                if req.get_path().ends_with(".ico") {
                    log::debug!("[main] favicon request, starting light detect");
                    let light_result = make_light_detect(&req, &config);
                    let mut request = req;
                    request = set_light_headers(request, light_result);
                    return Ok(request.send(APP_BACKEND).unwrap());
                }
                log::debug!("[main] path: {}, static requested => skipped bot detection", req.get_path());
                return Ok(req.send(APP_BACKEND).unwrap());
            }

            log::debug!("[main] path: {}, not static => do bot detection", req.get_path());
            return Ok(handle_request_with_bot_detect(req, &config))
        }
    }
}
