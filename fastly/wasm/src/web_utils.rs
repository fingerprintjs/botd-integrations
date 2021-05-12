use fastly::http::HeaderValue;
use fastly::Request;
use crate::constants::{SEC_FETCH_DEST_HEADER, STATIC_SEC_FETCH_DEST, STATIC_PATH_ENDINGS};

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