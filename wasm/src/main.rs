//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode, HeaderValue};
use fastly::{mime, Error, Request, Response};
use regex::Regex;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const APP_BACKEND: &str = "Ngrok";

/// The name of a second backend associated with this service.
const FPJS_BACKEND: &str = "Fpjs";
const FPJS_URL: &str = "https://fpjs-botd-dev-use1.fpjs.sh/api/v1/results"; // TODO: change to prod
const FPJS_TOKEN: &str = "JzdWIiOiIxMjM0NTY3O";

const BOT_STATUS_HEADER: &str = "fpjs-bot-status";
const REQUEST_STATUS_HEADER: &str = "fpjs-request-status";
const BOT_PROB_HEADER: &str = "fpjs-bot-prob";
const REQUEST_ID_HEADER: &str = "fpjs-request-id";
const IS_BOT_HEADER: &str = "fpjs-is-bot";

const COOKIE_FPJS_NAME: &str = "botd-request-id=";
const COOKIE_HEADER: &str = "cookie";
const SCRIPT_CONNECT: &str = r#"<script async src="./dist/botd.umd.min.js" onload="getResults()"></script>"#;
const SCRIPT_BODY: &str = r#"
    <script>
        async function getResults() {
            const botdPromise = FPJSBotDetect.load({
            token: "JzdWIiOiIxMjM0NTY3O",
            endpoint: "https://fpjs-botd-dev-use1.fpjs.sh/api/v1",
            async: true,
        })
        const botd = await botdPromise
        const result = await botd.get({isPlayground: true})
        console.log(result)
        }
    </script>"#;

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
            }
        }
    } else {
        return Option::None;
    }
    return Option::Some(value);
}

struct AllowingRequestResult {
    allow_to_request: bool,
    request_id: String
}

fn allow_to_request(req: &Request) -> AllowingRequestResult {
    // Get fpjs request id from cookie header
    let cookie_option = get_header_value(req.get_header(COOKIE_HEADER));
    if cookie_option.is_none() {
        return AllowingRequestResult { allow_to_request: true, request_id: "".parse().unwrap() };
    }
    let cookie_value = cookie_option.unwrap();
    let cookie_element = extract_cookie_element(&*cookie_value, COOKIE_FPJS_NAME);
    if cookie_element.is_none() {
        return AllowingRequestResult { allow_to_request: true, request_id: "".parse().unwrap() };
    }
    let fpjs_request_id = cookie_element.unwrap();

    // Build request for bot detection
    let mut verify_request = Request::get(FPJS_URL);
    let mut query_str: String = "header&token=".to_owned();
    query_str.push_str(FPJS_TOKEN);
    query_str.push_str("&id=");
    query_str.push_str(fpjs_request_id.as_str());
    verify_request.set_query_str(query_str);

    // Send verify request
    let verify_response = verify_request.send(FPJS_BACKEND).unwrap();

    // Extract bot detection procedure status
    let bot_status_option = get_header_value(verify_response.get_header(BOT_STATUS_HEADER));
    if bot_status_option.is_none() {
        return AllowingRequestResult { allow_to_request: true, request_id: fpjs_request_id };
    }
    let bot_status = bot_status_option.unwrap();

    // Extract bot probability value and decide if it's a bot
    let mut is_bot = false;
    if bot_status.eq("ok") {
        let bot_prob_option = get_header_value(verify_response.get_header(BOT_PROB_HEADER));
        if bot_prob_option.is_none() {
            return AllowingRequestResult { allow_to_request: true, request_id: fpjs_request_id };
        }
        let bot_prob: f32 = bot_prob_option.unwrap().parse().unwrap();
        if bot_prob >= 0.5 {
            is_bot = true;
        }
    }

    // Send an error if it's a bot, otherwise go to app backend and return the response
    return AllowingRequestResult { allow_to_request: !is_bot, request_id: fpjs_request_id };
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
    req.set_header(header::HOST, "we-protect-your-privacy.ngrok.io"); // TODO: fix it

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
            Ok(req.send(APP_BACKEND)?)
        }
        "/dist/botd.umd.min.js" => {
            Ok(req.send(APP_BACKEND)?)
        }

        "/login" => {
            let result = allow_to_request(&req);
            if result.allow_to_request {
                Ok(req.send(APP_BACKEND)?
                    .with_header(IS_BOT_HEADER, "0")
                    .with_header(REQUEST_ID_HEADER, result.request_id.as_str()))
            } else {
                Ok(Response::from_status(900)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .with_header(IS_BOT_HEADER, "1")
                    .with_header(REQUEST_ID_HEADER, result.request_id.as_str()))
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
