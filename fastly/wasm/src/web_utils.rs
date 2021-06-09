use fastly::http::HeaderValue;
use fastly::Request;
use crate::constants::{SEC_FETCH_DEST_HEADER, STATIC_SEC_FETCH_DEST, STATIC_PATH_ENDINGS, COOKIE_HEADER};
use std::num::TryFromIntError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;

pub fn get_timestamp_ms() -> Result<i64, TryFromIntError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    return i64::try_from(timestamp);
}

pub fn get_cookie_from_request(req: &Request, name: &str) -> Option<String> {
    let cookie_op = extract_header_value(req.get_header(COOKIE_HEADER));
    return if cookie_op.is_none() {
        None
    } else {
        let cookie_value = cookie_op.unwrap();
        return extract_cookie_element(&*cookie_value, name);
    }
}

pub fn extract_header_value(h: Option<&HeaderValue>) -> Option<String> {
    if h.is_none() {
        return Option::None;
    }
    return Option::Some(h.unwrap().to_str().unwrap_or_default().to_string());
}

pub fn extract_cookie_element(cookie: &str, element_name: &str) -> Option<String> {
    let position_option = cookie.find(element_name);
    if position_option.is_none() {
        return Option::None;
    }

    let mut cookie_value: String = String::new();
    let position = position_option.unwrap() + element_name.len();
    for i in position..cookie.len() {
        let c = cookie.chars().nth(i).unwrap();
        if c == ' ' || c == ';' {
            break
        }
        cookie_value.push(c);
    }
    return Option::Some(cookie_value);
}

pub fn is_static_requested(req: &Request) -> bool {
    // sec-fetch-dest header shows which content was requested, but it works not in all web-browsers
    let sec_fetch_dest_option = extract_header_value(req.get_header(SEC_FETCH_DEST_HEADER.to_owned()));
    if sec_fetch_dest_option.is_some() {
        let sec_fetch_dest = sec_fetch_dest_option.unwrap();
        for s in &STATIC_SEC_FETCH_DEST {
            if sec_fetch_dest.eq(s) {
                return true;
            }
        }
        return false;
    }

    // sec-fetch-dest header doesn't exist => check by path ending
    for s in &STATIC_PATH_ENDINGS {
        if req.get_path().ends_with(s) {
            return true;
        }
    }
    return false;
}

pub fn get_host_from_url(mut url: String) -> Option<String> {
    let http = "http://";
    let https = "https://";

    let proto_len = if url.starts_with(http) {
        http.len()
    } else if url.starts_with(https) {
        https.len()
    } else {
        return None
    };
    // Removing protocol
    url = String::from(&url[proto_len..]);
    return Some(url);
}