use fastly::Request;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::detector::{Detect, check_resp, transfer_headers};
use crate::request_id::RequestId;
use crate::error::BotdError;
use fastly::http::Method;

pub struct BotDetector {
    pub request_id: String
}

impl Detect for BotDetector {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> {
        let request_id = match RequestId::from_cookie(req) {
            Some(r) => r,
            _ => return Err(BotdError::NoRequestIdInCookie)
        };
        let query = format!("header&token={}&id={}", config.token.to_owned(), request_id);
        log::debug!("[botd] request_id = {}, query: ?{}", request_id, query);
        let botd_resp = match req
            .clone_without_body()
            .with_method(Method::GET)
            .with_path("/api/v1/results")
            .with_query_str(query)
            .send(BOTD_BACKEND_NAME) {
            Ok(r) => r,
            Err(e) => return Err(BotdError::SendError(String::from(e.backend_name())))
        };
        if let Err(err) = check_resp(&botd_resp) { return Err(err) }
        transfer_headers(req, &botd_resp);
        Ok(BotDetector{ request_id })
    }
}