use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::utils::{get_cookie, get_timestamp_ms};
use fastly::Request;
use json::JsonValue;
use crate::{COOKIE_NAME};
use crate::detector::{Detect, BLOB, check_resp, transfer_headers};
use crate::endpoint::BotdEndpoint;

pub struct EdgeDetect {
    previous_request_id: String,
}

fn create_body(req: &Request, previous_request_id: &String) -> &str {
    let headers_names = req.get_header_names_str();
    let mut headers_json = JsonValue::new_object();

    for header_name in headers_names {
        let header_value = req.get_header_str(header_name);
        if let Some(h) = header_value {
            headers_json[header_name] = json::JsonValue::new_array();
            if let Err(e) = headers_json[header_name].push(h) {
                log::error!("[EdgeDetector.collect] Error: {}", e);
            }
        }
    }

    let timestamp = get_timestamp_ms();
    log::debug!("[EdgeDetector.collect] timestamp: {}", timestamp.to_owned());

    let mut json = JsonValue::new_object();
    json["headers"] = headers_json;
    json["path"] = req.get_path().into();
    json["previous_request_id"] = previous_request_id.into();
    json["timestamp"] = timestamp.into();

    log::debug!("[collect_from_initial_request] json: {}", json.dump());

    json.dump().as_str()
}

impl Detect for EdgeDetect {
    fn make(req: &mut Request, config: &Config) -> Result<Self, &str> {
        let previous_request_id = match get_cookie(req, COOKIE_NAME) {
            Some(r) => r.to_owned(),
            _ => ""
        };
        let endpoint = BotdEndpoint::new("/light");
        let body = create_body(req, &previous_request_id);
        let mut edge_request = Request::post(BLOB);
        edge_request.set_path(endpoint.path.as_str());
        edge_request.set_query_str("header");
        edge_request.set_body_text_plain(body);
        edge_request.set_header("Auth-Token", config.token.to_owned());

        let botd_resp = edge_request.send(BOTD_BACKEND_NAME).ok();

        let err = check_resp(&botd_resp);
        transfer_headers(req, botd_resp);

        Ok(EdgeDetect { previous_request_id })
    }

    fn get_request_id(&self) -> String {
        self.previous_request_id.to_owned()
    }
}