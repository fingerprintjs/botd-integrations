use crate::constants::{BOTD_LIGHT_PATH, BOTD_BACKEND, COOKIE_HEADER, COOKIE_NAME, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER, LIGHT_STATUS_HEADER, LIGHT_PROB_HEADER, LIGHT_TYPE_HEADER, BOT_PROB_HEADER, BOT_STATUS_HEADER, BOT_TYPE_HEADER, FAILED_STR};
use crate::config::Config;
use fastly::{Request, Response};
use json::JsonValue;
use crate::web_utils::{extract_header_value, extract_cookie_element};
use crate::result_item::{get_result_item, ResultItem};

pub struct LightDetectResult {
    pub(crate) status: String,
    pub(crate) id: String,
    light_result: ResultItem
}

fn get_cookie(req: &Request, name: &str) -> String {
    let cookie_option = extract_header_value(req.get_header(COOKIE_HEADER));
    return if cookie_option.is_none() {
        String::from("")
    } else {
        let cookie_value = cookie_option.unwrap();
        let cookie_element = extract_cookie_element(&*cookie_value, name);
        cookie_element.unwrap()
    }
}

fn collect_from_initial_request(req: &Request, config: &Config) -> Response {
    let request_id = get_cookie(req, COOKIE_NAME);
    let mut light_request = Request::post(config.botd_url.to_owned());
    light_request.set_path(BOTD_LIGHT_PATH);
    let headers_names = req.get_header_names_str();
    let mut json = JsonValue::new_object();
    let mut headers_json = JsonValue::new_object();

    for header_name in headers_names.into_iter() {
        headers_json[header_name] = req.get_header_str(header_name).unwrap().into();
    }

    json["headers"] = headers_json.into();
    json["path"] = req.get_path().into();
    json["previous_request_id"] = request_id.into();

    log::debug!("json: {}", json.dump());

    light_request.set_body_str(json.dump().as_str());
    let light_response = light_request.send(BOTD_BACKEND).unwrap();

    log::debug!("light url: {}", light_response.get_backend_request().unwrap().get_url_str());
    return light_response
}

pub fn set_light_headers(mut req: Request, detect_result: LightDetectResult) -> Request {
    req.set_header(REQUEST_ID_HEADER, detect_result.id);
    req.set_header(REQUEST_STATUS_HEADER, detect_result.status);
    req.set_header(BOT_PROB_HEADER, detect_result.light_result.probability.to_string());
    req.set_header(BOT_STATUS_HEADER, detect_result.light_result.status);
    req.set_header(BOT_TYPE_HEADER, detect_result.light_result.kind);
    return req;
}

pub fn make_light_detect(req: &Request, config: &Config) -> LightDetectResult {
    let detect_response = collect_from_initial_request(&req, &config);
    let id = detect_response.get_header_str(REQUEST_ID_HEADER).unwrap().to_string();
    let status = detect_response.get_header_str(REQUEST_STATUS_HEADER).unwrap().to_string();
    let mut light_result = get_result_item(&detect_response, LIGHT_STATUS_HEADER.to_owned(), LIGHT_PROB_HEADER.to_owned(), LIGHT_TYPE_HEADER.to_owned());

    if light_result.status == FAILED_STR {
        light_result = get_result_item(&detect_response, BOT_STATUS_HEADER.to_owned(), BOT_PROB_HEADER.to_owned(), BOT_TYPE_HEADER.to_owned());
    }

    log::debug!("fpjs-request-status: {}, fpjs-request-id: {}, fpjs-light-status: {}, fpjs-light-prob: {}, fpjs-light-type: {}",
                status, id, light_result.status, light_result.probability, light_result.kind);

    return LightDetectResult{status, id, light_result};
}