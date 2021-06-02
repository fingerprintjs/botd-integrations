use crate::constants::{BOTD_LIGHT_PATH, BOTD_BACKEND};
use crate::config::Config;
use fastly::{Request, Response};
use json::JsonValue;

pub fn collect_from_initial_request(req: &Request, config: &Config) -> Response {
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

    log::debug!("json: {}", json.dump());

    light_request.set_body_str(json.dump().as_str());
    let light_response = light_request.send(BOTD_BACKEND).unwrap();

    log::debug!("light url: {}", light_response.get_backend_request().unwrap().get_url_str());
    return light_response
}