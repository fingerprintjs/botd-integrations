use fastly::Request;
use crate::constants::*;
use crate::result_item::{get_result_item, ResultItem};
use crate::extractors::{extract_header_value, extract_cookie_element};

pub struct BotDetectionResult {
    pub request_id: String,
    pub request_status: String,

    pub bot: ResultItem,
    pub search_bot: ResultItem,
    pub vm: ResultItem,
    pub browser_spoofing: ResultItem,
}

pub fn detect(req: &Request, token: &str) -> BotDetectionResult {
    let mut result = BotDetectionResult {
        request_id: "".to_owned(),
        request_status: "".to_owned(),

        bot: ResultItem { ..Default::default() },
        search_bot: ResultItem { ..Default::default() },
        vm: ResultItem { ..Default::default() },
        browser_spoofing: ResultItem{ ..Default::default() },
    };

    // Get botd request id from cookie header
    let cookie_option = extract_header_value(req.get_header(COOKIE_HEADER));
    if cookie_option.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let cookie_value = cookie_option.unwrap();
    let cookie_element = extract_cookie_element(&*cookie_value, COOKIE_NAME);
    if cookie_element.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let request_id = cookie_element.unwrap();
    result.request_id = request_id.to_owned();

    // Build request for bot detection
    let mut verify_request = Request::get(BOTD_URL);
    let mut query_str: String = "header&token=".to_owned();
    query_str.push_str(token);
    query_str.push_str("&id=");
    query_str.push_str(request_id.as_str());
    verify_request.set_query_str(query_str);

    // Send verify request
    let verify_response = verify_request.send(BOTD_BACKEND).unwrap();

    // Check status code
    if !verify_response.get_status().is_success() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }

    // Extract request status
    let request_status_option = extract_header_value(verify_response.get_header(REQUEST_STATUS_HEADER));
    if request_status_option.is_none() {
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let request_status = request_status_option.unwrap();
    if !request_status.eq(OK_STR) {
        result.request_status = request_status;
        return result;
    }
    result.request_status = OK_STR.to_owned();

    // Extract bot detection status
    result.bot = get_result_item(&verify_response, BOT_STATUS_HEADER.to_owned(), BOT_PROB_HEADER.to_owned(), BOT_TYPE_HEADER.to_owned());

    // Extract search bot detection status
    result.search_bot = get_result_item(&verify_response, SEARCH_BOT_STATUS_HEADER.to_owned(), SEARCH_BOT_PROB_HEADER.to_owned(), SEARCH_BOT_TYPE_HEADER.to_owned());

    // Extract vm detection status
    result.vm = get_result_item(&verify_response, VM_STATUS_HEADER.to_owned(), VM_PROB_HEADER.to_owned(), VM_TYPE_HEADER.to_owned());

    // Extract browser spoofing detection status
    result.browser_spoofing = get_result_item(&verify_response, BROWSER_SPOOFING_STATUS_HEADER.to_owned(), BROWSER_SPOOFING_PROB_HEADER.to_owned(), "".to_owned());

    return result;
}