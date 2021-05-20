use fastly::{Request, Response};
use crate::constants::*;
use crate::result_item::{get_result_item, ResultItem};
use crate::web_utils::{extract_header_value, extract_cookie_element};
use crate::config::Config;
use std::any::Any;

struct BotDetectionResult {
    pub request_id: String,
    pub request_status: String,

    pub bot: ResultItem,
    pub search_bot: ResultItem,
    pub vm: ResultItem,
    pub browser_spoofing: ResultItem,
}

fn bot_detect(req: &Request, config: &Config) -> BotDetectionResult {
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
        log::error!("path: {}, cookie header cannot be found", req.get_path());
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let cookie_value = cookie_option.unwrap();
    let cookie_element = extract_cookie_element(&*cookie_value, COOKIE_NAME);
    if cookie_element.is_none() {
        log::error!("path: {}, cookie element cannot be found", req.get_path());
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let request_id = cookie_element.unwrap();
    result.request_id = request_id.to_owned();
    log::debug!("path: {}, requestId = {}", req.get_path(), request_id.to_owned());

    // Build request for bot detection
    let mut verify_request = Request::get(config.botd_results_url.to_owned());
    let mut query_str: String = "header&token=".to_owned();
    query_str.push_str(&*config.botd_token);
    query_str.push_str("&id=");
    query_str.push_str(request_id.as_str());
    verify_request.set_query_str(query_str.to_owned());

    // Send verify request
    let verify_response = verify_request.send(BOTD_BACKEND).unwrap();

    // Check status code
    if !verify_response.get_status().is_success() {
        log::error!("path: {}, verify status code: {}, link: {}?{}", req.get_path(),
                    verify_response.get_status(), config.botd_results_url.to_owned(), query_str.to_owned());
        result.request_status = FAILED_STR.to_owned();
        return result;
    }

    // Extract request status
    let request_status_option = extract_header_value(verify_response.get_header(REQUEST_STATUS_HEADER));
    if request_status_option.is_none() {
        log::error!("path: {}, request status cannot be found", req.get_path());
        result.request_status = FAILED_STR.to_owned();
        return result;
    }
    let request_status = request_status_option.unwrap();
    if !request_status.eq(OK_STR) {
        log::warn!("path: {}, request status is {}, but expected OK", req.get_path(), request_status);
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
    result.browser_spoofing = get_result_item(&verify_response, BROWSER_SPOOFING_STATUS_HEADER.to_owned(), BROWSER_SPOOFING_PROB_HEADER.to_owned(), BROWSER_SPOOFING_TYPE_HEADER.to_owned());

    return result;
}

pub fn handle_request_with_bot_detect(mut req: Request, config: &Config) -> Response {
    let result = bot_detect(&req, &config);

    req = req.with_header(REQUEST_ID_HEADER, result.request_id.to_owned());
    req = req.with_header(REQUEST_STATUS_HEADER, result.request_status.to_owned());
    log::debug!("path: {}, {}: {}, {}: {}", req.get_path(), REQUEST_ID_HEADER, result.request_id.to_owned(),
                REQUEST_STATUS_HEADER, result.request_status.to_owned());

    if result.request_status.eq(OK_STR) {
        // Set bot detection result to header
        req = req.with_header(BOT_STATUS_HEADER, result.bot.status.as_str());
        if result.bot.status.eq(OK_STR) {
            req = req.with_header(BOT_PROB_HEADER, format!("{:.2}", result.bot.probability));
            req = req.with_header(BOT_TYPE_HEADER, result.bot.kind.to_owned());
            log::debug!("path: {}, {}: {}, {}: {}, {}: {}", req.get_path(), BOT_STATUS_HEADER,
                        result.bot.status.as_str(), BOT_PROB_HEADER, result.bot.probability,
                        BOT_TYPE_HEADER, result.bot.kind.to_owned());
        } else {
            log::debug!("path: {}, {}: {}", req.get_path(), BOT_STATUS_HEADER, result.bot.status.as_str());
        }

        // Set search bot detection result to header
        req = req.with_header(SEARCH_BOT_STATUS_HEADER, result.search_bot.status.as_str());
        if result.search_bot.status.eq(OK_STR) {
            req = req.with_header(SEARCH_BOT_PROB_HEADER, format!("{:.2}", result.search_bot.probability));
            req = req.with_header(SEARCH_BOT_TYPE_HEADER, result.search_bot.kind.to_owned());
            log::debug!("path: {}, {}: {}, {}: {}, {}: {}", req.get_path(), SEARCH_BOT_STATUS_HEADER,
                        result.search_bot.status.as_str(), SEARCH_BOT_PROB_HEADER, result.search_bot.probability,
                        SEARCH_BOT_TYPE_HEADER, result.search_bot.kind.to_owned());
        } else {
            log::debug!("path: {}, {}: {}", req.get_path(), SEARCH_BOT_STATUS_HEADER, result.search_bot.status.as_str());
        }

        // Set vm detection result to header
        req = req.with_header(VM_STATUS_HEADER, result.vm.status.as_str());
        if result.vm.status.eq(OK_STR) {
            req = req.with_header(VM_PROB_HEADER, format!("{:.2}", result.vm.probability));
            req = req.with_header(VM_TYPE_HEADER, result.vm.kind.to_owned());
            log::debug!("path: {}, {}: {}, {}: {}, {}: {}", req.get_path(), VM_STATUS_HEADER,
                        result.vm.status.as_str(), VM_PROB_HEADER, result.vm.probability,
                        VM_TYPE_HEADER, result.vm.kind.to_owned());
        } else {
            log::debug!("path: {}, {}: {}", req.get_path(), VM_STATUS_HEADER, result.vm.status.as_str());
        }

        // Set browser spoofing detection result to header
        req = req.with_header(BROWSER_SPOOFING_STATUS_HEADER, result.browser_spoofing.status.as_str());
        if result.browser_spoofing.status.eq(OK_STR) {
            req = req.with_header(BROWSER_SPOOFING_PROB_HEADER, format!("{:.2}", result.browser_spoofing.probability));
            log::debug!("path: {}, {}: {}, {}: {}, {}: {}", req.get_path(), BROWSER_SPOOFING_STATUS_HEADER,
                        result.browser_spoofing.status.as_str(), BROWSER_SPOOFING_PROB_HEADER, result.browser_spoofing.probability,
                        BROWSER_SPOOFING_TYPE_HEADER, result.browser_spoofing.kind.to_owned());
        } else {
            log::debug!("path: {}, {}: {}", req.get_path(), BROWSER_SPOOFING_STATUS_HEADER, result.browser_spoofing.status.as_str());
        }
    }

    return req.send(APP_BACKEND).unwrap();
}