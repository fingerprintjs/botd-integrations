//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode, HeaderValue};
use fastly::{mime, Error, Request, Response};
use regex::Regex;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const APP_BACKEND: &str = "Backend";
const APP_HOST: &str = "botd-example-app.fpjs.sh";

/// The name of a second backend associated with this service.
const FPJS_BACKEND: &str = "Botd";
const FPJS_URL: &str = "https://botd.fpapi.io/api/v1/results";
const FPJS_TOKEN: &str = "JzdWIiOiIxMjM0NTY3O";

const REQUEST_ID_HEADER: &str = "fpjs-request-id";
const REQUEST_STATUS_HEADER: &str = "fpjs-request-status";

const BOT_STATUS_HEADER: &str = "fpjs-bot-status";
const BOT_PROB_HEADER: &str = "fpjs-bot-prob";
const BOT_TYPE_HEADER: &str = "fpjs-bot-type";

const SEARCH_BOT_STATUS_HEADER: &str = "fpjs-search-bot-status";
const SEARCH_BOT_PROB_HEADER: &str = "fpjs-search-bot-prob";
const SEARCH_BOT_TYPE_HEADER: &str = "fpjs-search-bot-type";

const INCONS_STATUS_HEADER: &str = "fpjs-incons-status";
const INCONS_PROB_HEADER: &str = "fpjs-incons-prob";

const VM_STATUS_HEADER: &str = "fpjs-vm-status";
const VM_PROB_HEADER: &str = "fpjs-vm-prob";
const VM_TYPE_HEADER: &str = "fpjs-vm-type";

const IS_BOT_HEADER: &str = "fpjs-is-bot";
const CHALLENGE_HEADER: &str = "fpjs-challenge-id";

const COOKIE_FPJS_NAME: &str = "botd-request-id=";
const COOKIE_HEADER: &str = "cookie";
const SCRIPT_CONNECT: &str = r#"<script async src="https://unpkg.com/@fpjs-incubator/botd-agent@0/dist/botd.umd.min.js" onload="getResults()"></script>"#;
const SCRIPT_BODY: &str = r#"
    <script>
        async function getResults() {
            const botdPromise = FPJSBotDetect.load({
            token: "JzdWIiOiIxMjM0NTY3O",
            async: true,
        })
        const botd = await botdPromise
        const result = await botd.get({isPlayground: true})
        }
    </script>"#;

const FORBIDDEN_BODY: &str = "{\"error\": {\"code\": 403, \"description\": \"Forbidden\"}}";

const FAILED_STR: &str = "failed";
const OK_STR: &str = "ok";

fn add_fpjs_script(html: Box<str>) -> String {
    let mut fpjs_html = String::from(html);
    let head_close_re = Regex::new(r"(</head.*>)").unwrap();
    let connect_index = head_close_re.find(&*fpjs_html).unwrap().start();
    fpjs_html.insert_str(connect_index, SCRIPT_CONNECT);
    let body_open_re = Regex::new(r"(<body.*>)").unwrap();
    let script_index = body_open_re.find(&*fpjs_html).unwrap().end();
    fpjs_html.insert_str(script_index, SCRIPT_BODY);
    return fpjs_html;
}

fn get_header_value(h: Option<&HeaderValue>) -> Option<String> {
    if h.is_none() {
        return Option::None;
    }
    return Option::Some(h.unwrap().to_str().unwrap().parse().unwrap());
}

fn extract_cookie_element(cookie: &str, element_name: &str) -> Option<String> {
    let position = cookie.find(element_name);
    let mut value: String = String::new();
    if position.is_some() {
        let pos = position.unwrap() + element_name.len();
        for i in pos..cookie.len() {
            let ch = cookie.chars().nth(i).unwrap();
            if ch != ' ' && ch != ';' {
                value.push(ch);
            } else {
                break;
            }
        }
    } else {
        return Option::None;
    }
    return Option::Some(value);
}

struct SingleResult {
    status: String,
    probability: f64,
    kind: String
}

impl Default for SingleResult {
    fn default() -> SingleResult {
        SingleResult {
            status: "".to_owned(),
            probability: -1.0,
            kind: "".to_owned()
        }
    }
}

struct BotDetectionResult {
    request_id: String,
    request_status: String,

    bot_detection: SingleResult,
    search_bot_detection: SingleResult,
    vm_detection: SingleResult,
    inconsistency_detection: SingleResult,
}

fn get_single_result(verify_response: &Response, status_header: String, prob_header: String, kind_header: String) -> SingleResult {
    let mut result = SingleResult{
        status: "".to_string(),
        probability: -1.0,
        kind: "".to_string()
    };

    let status_option = get_header_value(verify_response.get_header(status_header));
    if status_option.is_none() {
        result.status = FAILED_STR.to_owned();
        return result;
    }
    let status = status_option.unwrap();

    if status.eq(OK_STR) {
        // Extract probability
        let prob_option = get_header_value(verify_response.get_header(prob_header));
        if prob_option.is_none() {
            result.status = FAILED_STR.to_owned();
            return result;
        }
        result.status = OK_STR.to_owned();
        result.probability = prob_option.unwrap().parse().unwrap();

        // Extract bot type
        if kind_header.len() == 0 {
            return result;
        }
        let type_option = get_header_value(verify_response.get_header(kind_header));
        if type_option.is_none() {
            return result;
        }
        result.kind = type_option.unwrap().parse().unwrap();
        return result;
    } else {
        result.status = status;
    }
    return result;
}

fn bot_detection(req: &Request) -> BotDetectionResult {
    let mut result = BotDetectionResult {
        request_id: "".to_owned(),
        request_status: "".to_owned(),

        bot_detection: SingleResult { ..Default::default() },
        search_bot_detection: SingleResult { ..Default::default() },
        vm_detection: SingleResult { ..Default::default() },
        inconsistency_detection: SingleResult{ ..Default::default() },
    };

    // Get fpjs request id from cookie header
    let cookie_option = get_header_value(req.get_header(COOKIE_HEADER));
    if cookie_option.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let cookie_value = cookie_option.unwrap();
    let cookie_element = extract_cookie_element(&*cookie_value, COOKIE_FPJS_NAME);
    if cookie_element.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let fpjs_request_id = cookie_element.unwrap();
    result.request_id = fpjs_request_id.to_owned();

    // Build request for bot detection
    let mut verify_request = Request::get(FPJS_URL);
    let mut query_str: String = "header&token=".to_owned();
    query_str.push_str(FPJS_TOKEN);
    query_str.push_str("&id=");
    query_str.push_str(fpjs_request_id.as_str());
    verify_request.set_query_str(query_str);

    // Send verify request
    let verify_response = verify_request.send(FPJS_BACKEND).unwrap();

    // Check status code
    if !verify_response.get_status().is_success() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }

    // Extract request status
    let request_status_option = get_header_value(verify_response.get_header(REQUEST_STATUS_HEADER));
    if request_status_option.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let request_status = request_status_option.unwrap();
    if !request_status.eq(OK_STR) {
        result.request_status = request_status;
        return result;
    }
    result.request_status = OK_STR.to_owned();

    // Extract bot detection status
    result.bot_detection = get_single_result(&verify_response, BOT_STATUS_HEADER.to_owned(), BOT_PROB_HEADER.to_owned(), BOT_TYPE_HEADER.to_owned());

    // Extract search bot detection status
    result.search_bot_detection = get_single_result(&verify_response, SEARCH_BOT_STATUS_HEADER.to_owned(), SEARCH_BOT_PROB_HEADER.to_owned(), SEARCH_BOT_TYPE_HEADER.to_owned());

    // Extract vm detection status
    result.vm_detection = get_single_result(&verify_response, VM_STATUS_HEADER.to_owned(), VM_PROB_HEADER.to_owned(), VM_TYPE_HEADER.to_owned());

    // Extract inconsistency detection status
    result.inconsistency_detection = get_single_result(&verify_response, INCONS_STATUS_HEADER.to_owned(), INCONS_PROB_HEADER.to_owned(), "".to_owned());

    return result;
}

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
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
            let new_response = add_fpjs_script(Box::from(response.into_body_str()));

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
            let result = bot_detection(&req);

            // Decision should we block the request or not
            let botd_calculated = result.request_status.eq(OK_STR)
                && result.bot_detection.status.eq(OK_STR);
            let is_bot = botd_calculated && result.bot_detection.probability >= 0.5;

            return if is_bot {
                req = req.with_header(REQUEST_ID_HEADER, result.request_id);

                // Set bot detection result to header
                req = req.with_header(BOT_STATUS_HEADER, result.bot_detection.status.as_str());
                if result.bot_detection.status.eq(OK_STR) {
                    req = req.with_header(BOT_PROB_HEADER, format!("{:.2}", result.bot_detection.probability));
                    req = req.with_header(BOT_TYPE_HEADER, result.bot_detection.kind);
                }

                // Set search bot detection result to header
                req = req.with_header(SEARCH_BOT_STATUS_HEADER, result.search_bot_detection.status.as_str());
                if result.search_bot_detection.status.eq(OK_STR) {
                    req = req.with_header(SEARCH_BOT_PROB_HEADER, format!("{:.2}", result.search_bot_detection.probability));
                    req = req.with_header(SEARCH_BOT_TYPE_HEADER, result.search_bot_detection.kind);
                }

                // Set vm detection result to header
                req = req.with_header(VM_STATUS_HEADER, result.vm_detection.status.as_str());
                if result.vm_detection.status.eq(OK_STR) {
                    req = req.with_header(VM_PROB_HEADER, format!("{:.2}", result.vm_detection.probability));
                    req = req.with_header(VM_TYPE_HEADER, result.vm_detection.kind);
                }

                // Set inconsistency detection result to header
                req = req.with_header(INCONS_STATUS_HEADER, result.inconsistency_detection.status.as_str());
                if result.inconsistency_detection.status.eq(OK_STR) {
                    req = req.with_header(INCONS_PROB_HEADER, format!("{:.2}", result.inconsistency_detection.probability));
                }

                // Change body of request
                req.set_body(FORBIDDEN_BODY);

                // Send request to backend
                req.send(APP_BACKEND);

                // Return 403 to client
                Ok(Response::from_status(StatusCode::FORBIDDEN)
                    .with_body(FORBIDDEN_BODY))
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
