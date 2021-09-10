use fastly::Request;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;
use fastly::http::header::CONTENT_TYPE;
use fastly::convert::ToMethod;

const SEC_FETCH_DEST_HEADER: &str = "sec-fetch-dest";
const COOKIE_HEADER: &str = "cookie";

// TODO: add all static types
const STATIC_SEC_FETCH_DEST: [&str; 7] = ["font", "script", "image", "style", "video", "manifest", "object"];

// TODO: add all static types
const STATIC_PATH_ENDINGS: [&str; 7] = [".css", ".js", ".jpg", ".png", ".svg", ".jpeg", ".woff2"];

pub fn remove_trailing_slash(src: &mut String) { if src.ends_with('/') { src.pop(); } }

pub fn create_req(method: impl ToMethod) -> Request { Request::new(method, "") }

pub fn get_timestamp_ms() -> i64 {
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => t.as_millis(),
        Err(e) => {
            log::error!("[get_timestamp_ms] {}", e);
            return -1;
        }
    };
    return match i64::try_from(timestamp) {
        Ok(t) => t,
        Err(e) => {
            log::error!("[get_timestamp_ms] {}", e);
            -1
        }
    };
}

pub fn get_cookie(req: &Request, name: &str) -> Option<String> {
    let cookie = req.get_header(COOKIE_HEADER)?.to_str().ok()?;
    let position = cookie.find(name)?;

    let mut cookie_value: String = String::new();
    let position = position + name.len();
    for i in position..cookie.len() {
        let c = cookie.chars().nth(i)?;
        if c == ' ' || c == ';' {
            break;
        }
        cookie_value.push(c);
    }
    Some(cookie_value)
}

pub fn is_static_requested(req: &Request) -> bool {
    // sec-fetch-dest header shows which content was requested, but it works not in all web-browsers
    let sec_fetch_dest_op = req.get_header(String::from(SEC_FETCH_DEST_HEADER));
    if let Some(sec_fetch_dest) = sec_fetch_dest_op {
        for s in STATIC_SEC_FETCH_DEST.iter() {
            if sec_fetch_dest.eq(s) {
                return true;
            }
        }
        return false;
    }

    // sec-fetch-dest header doesn't exist => check by path ending
    for s in STATIC_PATH_ENDINGS.iter() {
        if req.get_path().ends_with(s) {
            return true;
        }
    }
    false
}

pub fn is_html(req: &Request) -> bool {
    if let Some(h) = req.get_header(CONTENT_TYPE) {
        if let Ok(s) = h.to_str() {
            s.to_lowercase().contains("text/html")
        }
    }
    false
}

pub fn get_host_from_url(url: &String) -> Option<String> {
    let http = "http://";
    let https = "https://";

    let proto_len = if url.starts_with(http) {
        http.len()
    } else if url.starts_with(https) {
        https.len()
    } else {
        return None;
    };
    // Removing protocol
    let host = String::from(&url[proto_len..]);
    Some(host)
}

pub fn get_ip(req: &Request) -> String {
    return match req.get_client_ip_addr() {
        Some(t) => t.to_string(),
        None => "0.0.0.0".to_string()
    };
}