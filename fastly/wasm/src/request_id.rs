use fastly::{Request, Response};
use crate::REQUEST_ID_HEADER_COOKIE;
use crate::error::BotdError;
use crate::utils::get_cookie;

pub struct RequestId;

impl RequestId {
    pub fn from_body(botd_resp: Response) -> Option<String> {
        let body = botd_resp.into_body_str();
        let json = json::parse(body.as_str()).ok()?;
        if json.is_object() {
            return json["requestId"].to_owned().take_string()
        }
        None
    }

    pub fn from_header(resp: &Response) -> Result<String, BotdError> {
        let req_id_header = match resp.get_header(REQUEST_ID_HEADER_COOKIE) {
            Some(r) => r,
            _ => return Err(BotdError::NoRequestIdInHeaders)
        };
        return match req_id_header.to_str().ok() {
            Some(s) => Ok(String::from(s)),
            _ => Err(BotdError::ToStringCast(String::from("request id")))
        };
    }

    pub fn from_cookie(req: &Request) -> Option<String> {
        get_cookie(req, REQUEST_ID_HEADER_COOKIE)
    }
}