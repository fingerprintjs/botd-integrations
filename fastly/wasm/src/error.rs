use crate::{REQUEST_ID_HEADER_COOKIE, REQUEST_STATUS_HEADER, ERROR_DESCRIPTION_HEADER};
use crate::config::{APP_BACKEND_NAME, Config};
use crate::utils::{get_timestamp_ms, get_ip};
use crate::request_id::RequestId;
use fastly::{Request, Response, Error};
use json::JsonValue;
use fastly::http::request::SendError as FastlySendError;
use std::panic::PanicInfo;

const ROLLBAR_PANIC_TOKEN: &str = "36cf6b17b0ec46948ca419760be7dcbf";
const ROLLBAR_ERROR_TOKEN: &str = "089746dbc251481fac8e525e68c4bc17";

/// An error that occurred during bot detection
pub enum BotdError {
    /// A regex syntax error.
    RegexSyntax(String),
    /// Passed HTML string doesn't contain <head> tag
    WrongHTML,
    /// Can't extract botd token.
    NoTokenInConfig,
    /// Passed HTML string doesn't contain <head> tag
    Disabled,
    /// Can't extract botd request id from headers.
    NoRequestIdInHeaders,
    /// Can't extract botd request status from headers.
    NoRequestStatusInHeaders,
    /// Can't extract botd error descriptions from headers.
    NoErrorDescriptionInHeaders,
    /// Can't cast to string.
    ToStringCast(String),
    /// Error during request sending.
    SendError(FastlySendError),
    /// Can't extract botd request status from headers.
    NoRequestIdInCookie,
}

impl ToString for BotdError {
    fn to_string(&self) -> String {
        match self {
            BotdError::RegexSyntax(re) => format!("Can't create regex {}", re),
            BotdError::WrongHTML => String::from("Can't find head tag in response body"),
            BotdError::NoTokenInConfig => String::from("Can't get botd token from config"),
            BotdError::Disabled => String::from("Bot detection disabled"),
            BotdError::NoRequestIdInHeaders => String::from("Request id cannot be found in headers"),
            BotdError::NoRequestStatusInHeaders => String::from("Request status cannot be found in headers"),
            BotdError::NoErrorDescriptionInHeaders => String::from("Request status is not processed, but error description cannot be found."),
            BotdError::ToStringCast(name) => format!("Can't cast {} to string", name),
            BotdError::SendError(e) => format!("Error occurred during sending to backend: {}", e.root_cause()),
            BotdError::NoRequestIdInCookie => String::from("Request id cannot be found in cookie"),
        }
    }
}

fn send_error_to_app(req: Request, err: &BotdError, req_id: Option<String>) -> Result<Response, Error> {
    log::error!("[error] To application: {}, {}", err.to_string(), req.get_method_str());
    Ok(req
        .with_header(REQUEST_ID_HEADER_COOKIE, req_id.unwrap_or_default())
        .with_header(REQUEST_STATUS_HEADER, "error")
        .with_header(ERROR_DESCRIPTION_HEADER, err.to_string())
        .send(APP_BACKEND_NAME)?)
}

pub fn handle_error(
    mut req: Request,
    err: BotdError,
    config: Option<&Config>,
    send_to_app: bool
) -> Result<Response, Error> {
    log::error!("[error] Handled error");
    let req_id = RequestId::search_in_req(&mut req);
    let (token, ip) = match config {
        Some(c) => (c.token.to_owned(), c.ip.to_owned()),
        _ => (String::new(), get_ip(&req))
    };
    send_error_to_rollbar(token, ip, req_id.to_owned(), &err);
    if send_to_app {
        return send_error_to_app(req, &err, req_id);
    }
    let err_msg = format!("Error occurred during bot detection: {}", err.to_string());
    Err(Error::msg(err_msg))
}

fn send_error_to_rollbar(token: String,
                         ip: String,
                         req_id: Option<String>,
                         err: &BotdError) {
    let mut json = JsonValue::new_object();
    json["token"] = token.into();
    json["ip"] = ip.into();
    json["error"] = err.to_string().into();
    json["request_id"] = req_id.into();
    let msg = json.dump();
    let body = make_rollbar_body(msg.as_str(), "warning");
    log::error!("[error] Sending error to rollbar: {}", body);

    send_to_rollbar(body, ROLLBAR_ERROR_TOKEN)
}

fn make_rollbar_body(msg: &str, level: &str) -> String {
    // Rollbar request body structure
    //  {
    //      "data": {
    //          "environment": "fastly-production",
    //          "level": "info", // optional "error" by default
    //          "timestamp": "111111111", // optional when this occurred, as a unix timestamp.
    //          "body": {
    //              "message": {
    //                  "body": "Test info message by POST request"
    //              }
    //          }
    // }
    const ROLLBAR_ENV: &str = "fastly-production";
    let timestamp: i64 = get_timestamp_ms();
    let mut json = JsonValue::new_object();
    let mut json_data = JsonValue::new_object();
    json_data["environment"] = ROLLBAR_ENV.into();
    json_data["level"] = level.into();
    let mut json_body = JsonValue::new_object();
    let mut json_message = JsonValue::new_object();
    json_message["body"] = msg.into();
    json_body["message"] = json_message;
    json_data["body"] = json_body;
    json_data["timestamp"] = timestamp.into();
    json["data"] = json_data;

    json.dump()
}

fn send_to_rollbar(body: String, token: &str) {
    const ROLLBAR_BACKEND_NAME: &str = "rollbar";
    const ROLLBAR_PATH: &str = "/api/1/item/";
    const ROLLBAR_TOKEN_HEADER: &str = "X-Rollbar-Access-Token";

    if let Err(e) = Request::post("https://api.rollbar.com/")
        .with_path(ROLLBAR_PATH)
        .with_body(body)
        .with_header(ROLLBAR_TOKEN_HEADER, token)
        .send(ROLLBAR_BACKEND_NAME) {
        log::error!("[error] Error during sending error to rollbar: {}", e.root_cause());
    }
}

pub fn panic_hook() -> Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send> {
    Box::new(|e| {
        let mut json = JsonValue::new_object();
        json["timestamp"] = get_timestamp_ms().into();
        json["message"] = e.to_string().into();

        let body = make_rollbar_body(json.dump().as_str(), "error");
        log::error!("[error] Sending panic to rollbar: {}", body);

        send_to_rollbar(body, ROLLBAR_PANIC_TOKEN);
    })
}