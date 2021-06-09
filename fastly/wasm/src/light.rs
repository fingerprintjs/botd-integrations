use crate::constants::{BOTD_LIGHT_PATH, BOTD_BACKEND, COOKIE_NAME, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER, LIGHT_STATUS_HEADER, LIGHT_PROB_HEADER, LIGHT_TYPE_HEADER, BOT_PROB_HEADER, BOT_STATUS_HEADER, BOT_TYPE_HEADER, FAILED_STR};
use crate::config::Config;
use fastly::{Request, Response};
use json::JsonValue;
use crate::web_utils::{get_cookie_from_request, get_timestamp_ms};
use crate::result_item::{get_result_item, ResultItem};

pub struct LightDetectResult {
    pub(crate) status: String,
    pub(crate) id: String,
    light_result: ResultItem
}

pub fn set_light_headers(mut req: Request, detect_result: LightDetectResult) -> Request {
    req.set_header(REQUEST_ID_HEADER, detect_result.id);
    req.set_header(REQUEST_STATUS_HEADER, detect_result.status);
    req.set_header(BOT_PROB_HEADER, format!("{:.2}", detect_result.light_result.probability));
    req.set_header(BOT_STATUS_HEADER, detect_result.light_result.status);
    req.set_header(BOT_TYPE_HEADER, detect_result.light_result.kind);
    return req;
}

fn collect_from_initial_request(req: &Request, config: &Config) -> Response {
    let request_id_op = get_cookie_from_request(req, COOKIE_NAME);
    let request_id: String;
    if request_id_op.is_none() {
        request_id = String::from("")
    } else {
        request_id = request_id_op.unwrap();
    }

    log::debug!("[collect_from_initial_request] cookie {}{}", COOKIE_NAME, request_id);

    let headers_names = req.get_header_names_str();
    let mut headers_json = JsonValue::new_object();

    for header_name in headers_names.into_iter() {
        let header_value = req.get_header_str(header_name);
        if header_value.is_some() {
            headers_json[header_name] = json::JsonValue::new_array();
            let _result = headers_json[header_name].push(header_value.unwrap());
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
    let detect_response = collect_from_initial_request(&req, &config);

    let id = detect_response.get_header_str(REQUEST_ID_HEADER).unwrap().to_string();
    let status = detect_response.get_header_str(REQUEST_STATUS_HEADER).unwrap().to_string();

    log::debug!("[make_light_detect] id: {}, status: {}", id, status);

    let mut light_result = get_result_item(
        &detect_response,
        LIGHT_STATUS_HEADER.to_owned(),
        LIGHT_PROB_HEADER.to_owned(),
        LIGHT_TYPE_HEADER.to_owned());

    if light_result.status == FAILED_STR {
        light_result = get_result_item(
            &detect_response,
            BOT_STATUS_HEADER.to_owned(),
            BOT_PROB_HEADER.to_owned(),
            BOT_TYPE_HEADER.to_owned());
    }

    log::debug!("[make_light_detect] fpjs-request-status: {}, fpjs-request-id: {}, \
    fpjs-light-status: {}, fpjs-light-prob: {}, fpjs-light-type: {}",
                status, id, light_result.status, light_result.probability, light_result.kind);

    return LightDetectResult{status, id, light_result};
}