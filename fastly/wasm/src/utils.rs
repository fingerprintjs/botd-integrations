use fastly::Request;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;
use cookie::{Cookie, SameSite};
use std::net::{Ipv4Addr, IpAddr};
use IpAddr::V4;
use psl::{Psl, List};
use std::str::FromStr;

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

pub fn make_cookie(name: &str, value: String, domain: Option<String>) -> String {
    let mut cookie = Cookie::build(name, value)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();
    if let Some(d) = domain {
        log::debug!("[cookie] Cookie domain set to: {}", d);
        cookie.set_domain(d);
    }
    cookie.to_string()
}

pub fn get_e_tld_plus_one(req: &Request) -> Option<String> {
    let host = get_host(req)?;
    let is_ip = is_ip(host.to_owned());
    if is_ip {
        log::debug!("[domain] Host: {}, is ip address", host);
        return None;
    }
    log::debug!("[domain] Host: {}, is not ip address", host);
    let domain = List.domain(host.as_bytes())?;
    let result = std::str::from_utf8(domain.as_bytes()).ok()?;
    log::debug!("[domain] eTLD+1: {}", result);
    Some(String::from(result))
}

pub fn get_host(req: &Request) -> Option<String> {
    Some(req.get_url().host()?.to_string())
}

pub fn is_ip(ip_str: String) -> bool {
    IpAddr::from_str(ip_str.as_str()).is_ok()
}

pub fn get_ip(req: &Request) -> String {
    req
        .get_client_ip_addr()
        .unwrap_or(V4(Ipv4Addr::UNSPECIFIED))
        .to_string()
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

pub fn is_favicon_requested(req: &Request) -> bool {
    return is_static_requested(req) && req.get_path().ends_with(".ico");
}

// pub fn is_html(req: &Request) -> bool {
//     if let Some(h) = req.get_header(CONTENT_TYPE) {
//         if let Ok(s) = h.to_str() {
//             return s.to_lowercase().contains("text/html");
//         }
//     }
//     false
// }