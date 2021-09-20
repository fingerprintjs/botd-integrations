use fastly::Request;
use json::JsonValue;
use crate::utils::get_timestamp_ms;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::detector::{Detect, check_botd_resp, transfer_headers};
use crate::request_id::RequestId;
use crate::error::BotdError;
use fastly::http::Method;
use BotdError::SendError;

pub struct EdgeDetect {
    pub req_id: String,
}

impl EdgeDetect {
    fn create_body(req: &Request) -> String {
        let prev_req_id = RequestId::from_req_cookie(req).unwrap_or_default();
        log::debug!("[edge] Previous request id: {}", prev_req_id);
        let headers_names = req.get_header_names_str();
        let mut headers_json = JsonValue::new_object();
        for header_name in headers_names {
            let header_value = req.get_header_str(header_name);
            if let Some(h) = header_value {
                headers_json[header_name] = json::JsonValue::new_array();
                if let Err(e) = headers_json[header_name].push(h) { log::error!("[error] {}", e.to_string()); }
            }
        }
        let timestamp = get_timestamp_ms();
        let mut json = JsonValue::new_object();
        json["headers"] = headers_json;
        json["path"] = req.get_path().into();
        json["previous_request_id"] = prev_req_id.into();
        json["timestamp"] = timestamp.into();
        json.dump()
    }
}

impl Detect for EdgeDetect {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> {
        let body = EdgeDetect::create_body(req);
        let edge_resp = match req.clone_without_body()
            .with_method(Method::POST)
            .with_path("/api/v1/edge")
            .with_query_str("header")
            .with_body_text_plain(body.as_str())
            .with_header("Auth-Token", config.token.to_owned())
            .send(BOTD_BACKEND_NAME) {
            Ok(r) => r,
            Err(e) => return Err(SendError(e.root_cause().to_string()))
        };
        check_botd_resp(&edge_resp)?;
        let req_id = RequestId::from_resp_header(&edge_resp)?;
        transfer_headers(req, &edge_resp);
        Ok(EdgeDetect { req_id })
    }
}