use fastly::Request;
use json::JsonValue;
use crate::REQUEST_ID_HEADER_COOKIE;
use crate::utils::{get_cookie, get_timestamp_ms};
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::detector::{Detect, check_resp, transfer_headers, get_request_id};
use crate::endpoint::BotdEndpoint;
use crate::error::BotdError;
use fastly::http::Method;

pub struct EdgeDetect {
    pub request_id: String,
}

impl EdgeDetect {
    fn create_body(req: &Request) -> String {
        let previous_request_id = match get_cookie(req, REQUEST_ID_HEADER_COOKIE) {
            Some(r) => r,
            _ => String::from("")
        };
        log::debug!("[edge] Previous request id: {}", previous_request_id);
        let headers_names = req.get_header_names_str();
        let mut headers_json = JsonValue::new_object();
        for header_name in headers_names {
            let header_value = req.get_header_str(header_name);
            if let Some(h) = header_value {
                headers_json[header_name] = json::JsonValue::new_array();
                if let Err(e) = headers_json[header_name].push(h) {
                    log::error!("[error] {}", e.to_string());
                }
            }
        }
        let timestamp = get_timestamp_ms();
        let mut json = JsonValue::new_object();
        json["headers"] = headers_json;
        json["path"] = req.get_path().into();
        json["previous_request_id"] = previous_request_id.into();
        json["timestamp"] = timestamp.into();
        json.dump()
    }
}

impl Detect for EdgeDetect {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> {
        let endpoint = BotdEndpoint::new("/light");
        let body = EdgeDetect::create_body(req);
        let edge_resp = match req.clone_without_body()
            .with_method(Method::POST)
            .with_path(endpoint.path.as_str())
            .with_query_str("header")
            .with_body_text_plain(body.as_str())
            .with_header("Auth-Token", config.token.to_owned())
            .send(BOTD_BACKEND_NAME) {
            Ok(r) => r,
            Err(e) => return Err(BotdError::SendError(String::from(e.backend_name())))
        };
        if let Err(err) = check_resp(&edge_resp) { return Err(err) }
        let request_id = get_request_id(&edge_resp)?;
        transfer_headers(req, &edge_resp);
        Ok(EdgeDetect { request_id })
    }
}