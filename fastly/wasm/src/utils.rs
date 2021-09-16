use fastly::Request;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;
use fastly::http::header::{COOKIE};
use cookie::{Cookie, SameSite};

pub fn get_timestamp_ms() -> i64 {
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => t.as_millis(),
        Err(e) => {
            log::error!("[error] {}", e);
            return -1;
        }
    };
    return match i64::try_from(timestamp) {
        Ok(t) => t,
        Err(e) => {
            log::error!("[error] {}", e);
            -1
        }
    };
}

pub fn make_cookie(name: String, value: String) -> String {
    Cookie::build(name, value)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .finish()
        .to_string()
}

pub fn get_cookie(req: &Request, name: &str) -> Option<String> {
    let cookies = req.get_header(COOKIE)?.to_str().ok()?.split(';');
    for c in cookies {
        let cookie = Cookie::parse(c).ok()?;
        if cookie.name() == name {
            return Some(String::from(cookie.value()))
        }
    }
    None
}

pub fn is_static_requested(req: &Request) -> bool {
    const SEC_FETCH_DEST_HEADER: &str = "sec-fetch-dest";
    // TODO: add all static types
    const STATIC_SEC_FETCH_DEST: [&str; 7] = ["font", "script", "image", "style", "video", "manifest", "object"];
    // TODO: add all static types
    const STATIC_PATH_ENDINGS: [&str; 7] = [".css", ".js", ".jpg", ".png", ".svg", ".jpeg", ".woff2"];
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

// pub fn is_html(req: &Request) -> bool {
//     if let Some(h) = req.get_header(CONTENT_TYPE) {
//         if let Ok(s) = h.to_str() {
//             return s.to_lowercase().contains("text/html");
//         }
//     }
//     false
// }