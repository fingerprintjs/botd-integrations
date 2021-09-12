use fastly::{Request, Response};
use crate::config::{Config};
use crate::{ERROR_DESCRIPTION, REQUEST_STATUS_HEADER};

pub const PROCESSED: &str = "processed";
pub const ERROR: &str = "error";

pub trait Detect {
    fn make(req: &mut Request, config: &Config) -> Result<Self, String> where Self: Sized;
}

pub fn transfer_headers(req: &mut Request, botd_resp: &Response) {
    static RESULT_HEADERS: [&str; 15] = [
        "botd-request-id",
        "botd-request-status",
        "botd-error-description",
        "botd-automation-tool-status",
        "botd-automation-tool-prob",
        "botd-automation-tool-type",
        "botd-search-bot-status",
        "botd-search-bot-prob",
        "botd-search-bot-type",
        "botd-browser-spoofing-status",
        "botd-browser-spoofing-prob",
        "botd-browser-spoofing-type",
        "botd-vm-status",
        "botd-vm-prob",
        "botd-vm-type"
    ];
    for header_name in RESULT_HEADERS {
        if let Some(header_value) = botd_resp.get_header(header_name) {
            req.set_header(header_name, header_value);
        };
    }
}

// fn get_request_id(resp: &Response) -> Result<String, String> {
//     let request_id_header = match resp.get_header(REQUEST_ID_HEADER) {
//         Some(r) => r,
//         _ => return Err(String::from("[Compute@Edge] Request id cannot be found."))
//     };
//
//     return match request_id_header.to_str() {
//         Ok(s) => Ok(String::from(s)),
//         Err(_e) => Err(String::from("[Compute@Edge] Can't cast request id header to string."))
//     };
// }

pub fn check_resp(resp: &Response) -> Result<(), String> {
    let request_status = match resp.get_header(REQUEST_STATUS_HEADER) {
        Some(r) => r,
        _ => return Err(String::from("Request status cannot be found."))
    };
    if !request_status.eq(PROCESSED) && resp.get_header(ERROR_DESCRIPTION).is_none() {
        return Err(String::from("Request status is not processed, but error description cannot be found."))
    }
    Ok(())
}