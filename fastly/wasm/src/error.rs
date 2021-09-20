use fastly::{Request, Response, Error};
use crate::{REQUEST_ID_HEADER_COOKIE, REQUEST_STATUS_HEADER, ERROR_DESCRIPTION_HEADER};
use crate::config::{APP_BACKEND_NAME, BOTD_BACKEND_NAME, Config};
use crate::utils::{get_timestamp_ms, get_ip};
use json::JsonValue;
use JsonValue::Null;
use crate::request_id::RequestId;
use fastly::http::Method;

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
    SendError(String),
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
            BotdError::SendError(desc) => format!("Error occurred during sending to backend: {}", desc),
            BotdError::NoRequestIdInCookie => String::from("Request id cannot be found in cookie"),
        }
    }
}

fn send_error_to_app(req: Request, err: &BotdError, req_id: Option<String>) -> Result<Response, Error> {
    log::error!("[error] To application: {}", err.to_string());
    Ok(req
        .with_header(REQUEST_ID_HEADER_COOKIE, req_id.unwrap_or_default())
        .with_header(REQUEST_STATUS_HEADER, "error")
        .with_header(ERROR_DESCRIPTION_HEADER, err.to_string())
        .send(APP_BACKEND_NAME)?)
}

fn send_error_to_botd(req: Request,
                      config: JsonValue,
                      ip_address: String,
                      req_id: Option<String>,
                      err: &BotdError) -> Result<Response, Error> {
    let timestamp = get_timestamp_ms();
    let mut json = JsonValue::new_object();
    json["config"] = config;
    json["ip_address"] = ip_address.into();
    json["error"] = err.to_string().into();
    json["request_id"] = req_id.into();
    json["timestamp"] = timestamp.into();
    let body = json.dump();
    log::error!("[error] To botd: {}", body);
    Ok(req
        .with_method(Method::POST)
        .with_path("/integration/error")
        .with_body(body)
        .send(BOTD_BACKEND_NAME)?)
}

pub fn handle_error(
    mut req: Request,
    err: BotdError,
    config: Option<&Config>,
    send_to_app: bool
) -> Result<Response, Error> {
    let req_id = RequestId::search_in_req(&mut req);
    let ip = get_ip(&req);
    let config = match config {
        Some(c) => c.json(),
        _ => Null
    };
    let mut resp = None;
    if send_to_app {
        let app_req = req.clone_with_body();
        resp = Some(send_error_to_app(app_req, &err, req_id.to_owned())?);
    }
    let botd_resp = send_error_to_botd(req, config, ip, req_id, &err)?;
    match resp {
        Some(r) => Ok(r),
        _ => Ok(botd_resp)
    }
}