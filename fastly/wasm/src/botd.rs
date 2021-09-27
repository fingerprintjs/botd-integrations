use fastly::Request;
use fastly::http::Method;
use BotdError::SendError;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::detector::{Detect, check_botd_resp, transfer_headers};
use crate::request_id::RequestId;
use crate::error::BotdError;
use crate::CLIENT_IP_HEADER;

pub struct BotDetector {
    pub req_id: String,
}

impl Detect for BotDetector {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> {
        let req_id = RequestId::from_req_cookie(req)?;
        let query = format!("header&token={}&id={}", config.token.to_owned(), req_id);
        log::debug!("[botd] Make bot detect with request_id: {} and query: ?{}", req_id, query);
        match req
            .clone_without_body()
            .with_method(Method::GET)
            .with_path("/api/v1/results")
            .with_query_str(query)
            .with_header(CLIENT_IP_HEADER, config.ip.to_owned())
            .send(BOTD_BACKEND_NAME) {
            Ok(r) => {
                check_botd_resp(&r)?;
                transfer_headers(req, &r);
                Ok(BotDetector { req_id })
            }
            Err(e) => Err(SendError(e))
        }
    }
}