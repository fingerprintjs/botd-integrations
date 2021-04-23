//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode, HeaderValue};
use fastly::{mime, Error, Request, Response};
use fastly::handle::client_request_and_body;
use regex::Regex;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const NGROK_BACKEND: &str = "Ngrok";

/// The name of a second backend associated with this service.
const FPJS_BACKEND: &str = "Fpjs";

fn add_script(html: Box<str>) -> String {
    let script_connect = r#"<script async src="./dist/botd.umd.min.js" onload="getResults()"></script>
    "#;
    let script_body = r#"
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
    let mut result_string = String::from(html);
    let head_close_re = Regex::new(r"(</head.*>)").unwrap();
    let connect_index = head_close_re.find(&*result_string).unwrap().start();
    result_string.insert_str(connect_index, script_connect);
    let body_open_re = Regex::new(r"(<body.*>)").unwrap();
    let script_index = body_open_re.find(&*result_string).unwrap().end();
    result_string.insert_str(script_index, script_body);
    return result_string;
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
    req.set_header(header::HOST, "we-protect-your-privacy.ngrok.io");

    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD | &Method::POST => (),

        // Accept PURGE requests; it does not matter to which backend they are sent.
        m if m == "PURGE" => return Ok(req.send(NGROK_BACKEND)?),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, POST")
                .with_body_str("This method is not allowed\n"))
        }
    };

    // Pattern match on the path.
    match req.get_path() {
        "/" => {
            // Request handling logic could go here...  E.g., send the request to an origin backend
            // and then cache the response for one minute.

            let response = req.send(NGROK_BACKEND).unwrap();
            let new_response = add_script(Box::from(response.into_body_str()));

            return Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(new_response));
        }
        "/img/favicon.ico" => {
            Ok(req.send(NGROK_BACKEND)?)
        }
        "/dist/botd.umd.min.js" => {
            Ok(req.send(NGROK_BACKEND)?)
        }

        "/login" => {
            let mut verify_request = Request::get("https://fpjs-botd-dev-use1.fpjs.sh/api/v1/results");

            let mut query_str: String = "header&token=JzdWIiOiIxMjM0NTY3O&id=".to_owned();

            let cookie_header_value = req
                .get_header("cookie").expect("no cookie header")
                .to_str().expect("error during header value to_string");

            let cookie_name = "botd-request-id=";
            let cookie_pos = cookie_header_value.find(cookie_name);
            let mut botd_request_id: String = String::new();
            if cookie_pos.is_some() {
                let pos = cookie_pos.unwrap() + cookie_name.len();
                for i in pos..cookie_header_value.len() {
                    let ch = cookie_header_value.chars().nth(i).unwrap();
                    if ch != ' ' && ch != ';' {
                        botd_request_id.push(ch);
                    }
                }
            }

            query_str.push_str(botd_request_id.as_str());

            verify_request.set_query_str(query_str);

            let verify_response = verify_request.send(FPJS_BACKEND).unwrap();
            let bot_status = verify_response
                .get_header("fpjs-bot-status").expect("no fpjs-request-status header")
                .to_str().expect("error during header value to_string");
            req.set_header("fpjs-request-status", bot_status);

            let mut is_bot = "undefined";
            if bot_status.eq("ok") {
                let bot_prob = verify_response
                    .get_header("fpjs-bot-prob").expect("no fpjs-bot-prob header")
                    .to_str().expect("error during header value to_string");
                is_bot = if bot_prob.eq("0.00") { "0" } else { "1" };
            }

            if is_bot.eq("1") {
                Ok(Response::from_status(StatusCode::IM_A_TEAPOT)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .with_header("fpjs-bot-status", bot_status)
                    .with_header("fpjs-is-bot", is_bot)
                    .with_header("request-id", botd_request_id.as_str()))
            } else {
                Ok(req.send(NGROK_BACKEND)?)
            }
        }

        // If request is to a path starting with `/other/`...
        path if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_pass(true);
            Ok(req.send(NGROK_BACKEND)?)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_str("The page you requested could not be found\n")),
    }

    // client_request_and_body() {
    //     req.with_body()
    // }
}
