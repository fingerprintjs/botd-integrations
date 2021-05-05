//! Default Compute@Edge template program.

mod injector;
mod extractors;
mod result_item;
mod bot_detector;
mod constants;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response, Dictionary};
use constants::*;
use bot_detector::detect;
use injector::add_bot_detection_script;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const APP_BACKEND: &str = "Backend";
const APP_HOST: &str = "botd-example-app.fpjs.sh";

const FORBIDDEN_BODY: &str = "{\"error\": {\"code\": 403, \"description\": \"Forbidden\"}}";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    let config = Dictionary::open("config");
    let token_option = config.get("token");
    if token_option.is_none() {
        return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_body_str("Token cannot be extracted from fastly configuration\n"))
    }
    let token = token_option.unwrap();

    // Make any desired changes to the client request.
    req.set_header(header::HOST, APP_HOST);

    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
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
    match req.get_path() {
        "/" => {
            req.set_pass(true); // TODO: get rid of it
            let response = req.send(APP_BACKEND).unwrap();
            let new_response = add_bot_detection_script(Box::from(response.into_body_str()), token.as_str());

            return Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(new_response));
        }
        "/img/favicon.ico" => {
            req.set_pass(true);
            Ok(req.send(APP_BACKEND)?)
        }
        "/login" => {
            req.set_pass(true); // TODO: get rid of it
            let result = detect(&req, token.as_str());

            // Decision should we block the request or not
            let botd_calculated = result.request_status.eq(OK_STR)
                && result.bot.status.eq(OK_STR);
            let is_bot = botd_calculated && result.bot.probability >= 0.5;

            return if is_bot {
                req = req.with_header(REQUEST_ID_HEADER, result.request_id);
                req = req.with_header(REQUEST_STATUS_HEADER, result.request_status);

                // Set bot detection result to header
                req = req.with_header(BOT_STATUS_HEADER, result.bot.status.as_str());
                if result.bot.status.eq(OK_STR) {
                    req = req.with_header(BOT_PROB_HEADER, format!("{:.2}", result.bot.probability));
                    req = req.with_header(BOT_TYPE_HEADER, result.bot.kind);
                }

                // Set search bot detection result to header
                req = req.with_header(SEARCH_BOT_STATUS_HEADER, result.search_bot.status.as_str());
                if result.search_bot.status.eq(OK_STR) {
                    req = req.with_header(SEARCH_BOT_PROB_HEADER, format!("{:.2}", result.search_bot.probability));
                    req = req.with_header(SEARCH_BOT_TYPE_HEADER, result.search_bot.kind);
                }

                // Set vm detection result to header
                req = req.with_header(VM_STATUS_HEADER, result.vm.status.as_str());
                if result.vm.status.eq(OK_STR) {
                    req = req.with_header(VM_PROB_HEADER, format!("{:.2}", result.vm.probability));
                    req = req.with_header(VM_TYPE_HEADER, result.vm.kind);
                }

                // Set browser spoofing detection result to header
                req = req.with_header(BROWSER_SPOOFING_STATUS_HEADER, result.browser_spoofing.status.as_str());
                if result.browser_spoofing.status.eq(OK_STR) {
                    req = req.with_header(BROWSER_SPOOFING_PROB_HEADER, format!("{:.2}", result.browser_spoofing.probability));
                }

                // Change body of request
                req.set_body(FORBIDDEN_BODY);

                // Send request to backend
                req.send(APP_BACKEND);

                // Return 403 to client
                Ok(Response::from_status(StatusCode::FORBIDDEN).with_body(FORBIDDEN_BODY))
            } else {
                // No bot => pass the request to backend
                Ok(req.send(APP_BACKEND)?)
            }
        }

        // If request is to a path starting with `/other/`...
        path if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_pass(true);
            Ok(req.send(APP_BACKEND)?)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_str("The page you requested could not be found\n")),
    }
}
