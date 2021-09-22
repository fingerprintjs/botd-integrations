use fastly::{Request, Response};
use BotdError::{NoRequestStatusInHeaders, NoErrorDescriptionInHeaders};
use crate::config::Config;
use crate::{ERROR_DESCRIPTION_HEADER, REQUEST_STATUS_HEADER};
use crate::error::BotdError;

pub trait Detect {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> where Self: Sized;
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

pub fn check_botd_resp(resp: &Response) -> Result<(), BotdError> {
    log::debug!("[check] Headers: {:?}", resp.get_header_names_str());
    let req_status = match resp.get_header(REQUEST_STATUS_HEADER) {
        Some(r) => r,
        _ => return Err(NoRequestStatusInHeaders)
    };
    if !req_status.eq("processed") && resp.get_header(ERROR_DESCRIPTION_HEADER).is_none() {
        return Err(NoErrorDescriptionInHeaders);
    }
    Ok(())
}