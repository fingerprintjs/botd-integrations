use fastly::Response;
use crate::web_utils::extract_header_value;
use crate::constants::{OK_STR, FAILED_STR};

pub struct ResultItem {
    pub status: String,
    pub probability: f64,
    pub kind: String
}

impl Default for ResultItem {
    fn default() -> ResultItem {
        ResultItem {
            status: "".to_owned(),
            probability: -1.0,
            kind: "".to_owned()
        }
    }
}

pub fn get_result_item(verify_response: &Response, status_header: String, prob_header: String, kind_header: String) -> ResultItem {
    let mut result = ResultItem { ..Default::default() };

    // Extract status
    let status_option = extract_header_value(verify_response.get_header(status_header.to_owned()));
    if status_option.is_none() {
        log::error!("[get_result_item] {} header cannot be found", status_header.to_owned());
        result.status = FAILED_STR.to_owned();
        return result;
    }
    let status = status_option.unwrap();
    if !status.eq(OK_STR) {
        log::error!("[get_result_item] request status is {}", status);
        result.status = status;
        return result;
    }

    // Extract probability
    let prob_option = extract_header_value(verify_response.get_header(prob_header.to_owned()));
    if prob_option.is_none() {
        log::error!("[get_result_item] {} header cannot be found", prob_header.to_owned());
        result.status = FAILED_STR.to_owned();
        return result;
    }
    result.status = OK_STR.to_owned();
    result.probability = prob_option.unwrap().parse().unwrap();

    // Extract bot type
    if kind_header.len() == 0 {
        return result;
    }
    let type_option = extract_header_value(verify_response.get_header(kind_header));
    if type_option.is_some() {
        result.kind = type_option.unwrap().parse().unwrap();
    }
    return result;
}