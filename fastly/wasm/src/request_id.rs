use fastly::{Request, Response};
use BotdError::{NoRequestIdInCookie, NoRequestIdInHeaders, ToStringCast};
use crate::REQUEST_ID_HEADER_COOKIE;
use crate::error::BotdError;
use fastly::http::header::COOKIE;
use cookie::Cookie;
use fastly::http::HeaderValue;

pub struct RequestId;

fn find_in_cookie_str(cookies: &str, name: &str) -> Option<String> {
    let cookies = cookies.split(';');
    for c in cookies {
        if let Ok(cookie) = Cookie::parse(c) {
            if cookie.name() == name {
                return Some(String::from(cookie.value()));
            }
        }
    }
    None
}

pub fn get_cookie(req: &Request, name: &str) -> Option<String> {
    let cookies = req.get_header(COOKIE)?.to_str().ok()?;
    find_in_cookie_str(cookies, name)
}

impl RequestId {
    fn extract_from_body_str(body: String) -> Option<String> {
        let json = json::parse(body.as_str()).ok()?;
        if json.is_object() {
            return json["requestId"].to_owned().take_string();
        }
        None
    }

    fn extract_from_header(header: &HeaderValue) -> Result<String, BotdError> {
        match header.to_str() {
            Ok(r) => Ok(String::from(r)),
            Err(_) => Err(ToStringCast(String::from("request id")))
        }
    }

    pub fn from_resp_body(resp: Response) -> Option<String> {
        let body = resp.into_body_str();
        Self::extract_from_body_str(body)
    }

    pub fn from_req_body(req: Request) -> Option<String> {
        let body = req.into_body_str();
        Self::extract_from_body_str(body)
    }

    pub fn from_resp_header(resp: &Response) -> Result<String, BotdError> {
        match resp.get_header(REQUEST_ID_HEADER_COOKIE) {
            Some(r) => Ok(Self::extract_from_header(r)?),
            _ => Err(NoRequestIdInHeaders)
        }
    }

    pub fn from_req_header(req: &Request) -> Result<String, BotdError> {
        return match req.get_header(REQUEST_ID_HEADER_COOKIE) {
            Some(r) => Ok(Self::extract_from_header(r)?),
            _ => Err(NoRequestIdInHeaders)
        };
    }

    pub fn from_req_cookie(req: &Request) -> Result<String, BotdError> {
        match get_cookie(req, REQUEST_ID_HEADER_COOKIE) {
            Some(r) => Ok(r),
            _ => Err(NoRequestIdInCookie)
        }
    }

    // fn from_resp_cookie(resp: &Response) -> Result<String, BotdError> {
    //     match resp.get_header(SET_COOKIE) {
    //         Some(r) => {
    //             let cookies = Self::extract_from_header(r)?;
    //             match find_in_cookie_str(cookies.as_str(), REQUEST_ID_HEADER_COOKIE) {
    //                 Some(r) => Ok(r),
    //                 _ => Err(NoRequestIdInCookie)
    //             }
    //         }
    //         _ => Err(NoRequestIdInHeaders)
    //     }
    // }

    pub fn search_in_req(req: &mut Request) -> Option<String> {
        let in_cookie = Self::from_req_cookie(req).ok();
        let in_header = Self::from_req_header(req).ok();
        let in_body = Self::from_req_body(req.clone_with_body());

        if in_cookie.is_some() { in_cookie }
        else if in_header.is_some() { in_header }
        else if in_body.is_some() { in_body }
        else { None }
    }

    // pub fn search_in_resp(resp: &mut Response) -> Option<String> {
    //     let in_cookie = Self::from_resp_cookie(resp).ok();
    //     let in_header = Self::from_resp_header(resp).ok();
    //     let in_body = Self::from_resp_body(resp.clone_with_body());
    //
    //     if in_cookie.is_some() { in_cookie }
    //     else if in_header.is_some() { in_header }
    //     else if in_body.is_some() { in_body }
    //     else { None }
    // }
}