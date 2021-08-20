use crate::constants::{BOTD_LIGHT_PATH, BOTD_BACKEND, COOKIE_NAME, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER, AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_TYPE_HEADER, ERROR, PROCESSED, ERROR_DESCRIPTION};
use crate::config::Config;
use fastly::{Request, Response};
use json::JsonValue;
use crate::web_utils::{get_cookie_from_request, get_timestamp_ms, extract_header_value};
use crate::result_item::{get_result_item, ResultItem};

pub struct LightDetectResult {
    pub request_id: String,
    pub request_status: String,
    pub error_description: String,

    automation_tool: ResultItem
}

pub fn set_light_headers(mut req: Request, detect_result: LightDetectResult) -> Request {
    req.set_header(REQUEST_ID_HEADER, detect_result.request_id);
    req.set_header(REQUEST_STATUS_HEADER, detect_result.request_status.to_owned());
    req.set_header(AUTO_TOOL_STATUS_HEADER, detect_result.automation_tool.status);
    if detect_result.request_status.eq(PROCESSED) {
        req.set_header(AUTO_TOOL_PROB_HEADER, format!("{:.2}", detect_result.automation_tool.probability));
    }

    req.set_header(AUTO_TOOL_TYPE_HEADER, detect_result.automation_tool.kind);
    return req;
}

fn collect_and_send(req: &Request, config: &Config) -> Response {
    let request_id_option = get_cookie_from_request(req, COOKIE_NAME);
    let request_id: String;
    if request_id_option.is_none() {
        request_id = String::from("")
    } else {
        request_id = request_id_option.unwrap();
    }

    log::debug!("[collect_from_initial_request] cookie {}{}", COOKIE_NAME, request_id);

    let headers_names = req.get_header_names_str();
    let mut headers_json = JsonValue::new_object();

    for header_name in headers_names.into_iter() {
        let header_value = req.get_header_str(header_name);
        if header_value.is_some() {
            headers_json[header_name] = json::JsonValue::new_array();
            headers_json[header_name].push(header_value.unwrap());
        }
    }

    let timestamp_res = get_timestamp_ms();
    log::debug!("[collect_from_initial_request] timestamp: {}", timestamp_res.unwrap().to_string());

    let mut json = JsonValue::new_object();
    json["headers"] = headers_json.into();
    json["path"] = req.get_path().into();
    json["previous_request_id"] = request_id.into();

    if timestamp_res.is_ok() {
        json["timestamp"] = timestamp_res.unwrap().into();
    } else {
        let default_timestamp: i64 = -1;
        json["timestamp"] = default_timestamp.into();
    }

    log::debug!("[collect_from_initial_request] json: {}", json.dump());

    let mut light_request = Request::post(config.botd_url.to_owned());
    light_request.set_path(BOTD_LIGHT_PATH);
    light_request.set_query_str("header");
    light_request.set_body_str(json.dump().as_str());
    light_request.set_header("Auth-Token", &config.botd_token);

    log::debug!("[collect_from_initial_request] light request method: {}, url: {}, path: {}, body: {}",
                light_request.get_method_str(),
                light_request.get_url_str(),
                light_request.get_path(),
                json.dump().as_str());

    let light_response = light_request.send(BOTD_BACKEND).unwrap();

    log::debug!("[collect_from_initial_request] light response status: {}, url: {}",
                light_response.get_status(),
                light_response.get_backend_request().unwrap().get_url_str());

    return light_response
}

pub fn make_light_detect(req: &Request, config: &Config) -> LightDetectResult {
    let mut result = LightDetectResult {
        request_id: "".to_owned(),
        request_status: "".to_owned(),
        error_description: "".to_owned(),

        automation_tool: ResultItem { ..Default::default() },
    };

    let detect_response = collect_and_send(&req, &config);

    let request_id_option = extract_header_value(detect_response.get_header(REQUEST_ID_HEADER));
    if request_id_option.is_none() {
        log::error!("[light_detect] path: {}, request id cannot be found", req.get_path());
        result.request_status = ERROR.to_owned();
        return result;
    }
    result.request_id = request_id_option.unwrap();

    let request_status_option = extract_header_value(detect_response.get_header(REQUEST_STATUS_HEADER));
    if request_status_option.is_none() {
        log::error!("[light_detect] path: {}, request status cannot be found", req.get_path());
        result.request_status = ERROR.to_owned();
        return result;
    }
    result.request_status = request_status_option.unwrap();

    if !result.request_status.eq(PROCESSED) {
        let error_option = extract_header_value(detect_response.get_header(ERROR_DESCRIPTION));
        if error_option.is_some() {
            result.error_description = error_option.unwrap()
        }
    }

    log::debug!("[make_light_detect] id: {}, status: {}", result.request_id, result.request_status);

    result.automation_tool = get_result_item(
        &detect_response,
        AUTO_TOOL_STATUS_HEADER.to_owned(),
        AUTO_TOOL_PROB_HEADER.to_owned(),
        AUTO_TOOL_TYPE_HEADER.to_owned());

    log::debug!("[make_light_detect] botd-request-status: {}, botd-request-id: {}, \
                botd-light-status: {}, botd-light-prob: {}, botd-light-type: {}",
                result.request_status, result.request_id, result.automation_tool.status,
                result.automation_tool.probability, result.automation_tool.kind);

    return result;
}