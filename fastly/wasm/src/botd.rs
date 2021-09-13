use fastly::Request;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::{REQUEST_ID_HEADER_COOKIE};
use crate::utils::get_cookie;
use crate::detector::{Detect, check_resp, transfer_headers};
use crate::endpoint::BotdEndpoint;
use crate::error::BotdError;

pub struct BotDetector {
    pub request_id: String
}

impl Detect for BotDetector {
    fn make(req: &mut Request, config: &Config) -> Result<Self, BotdError> {
        let request_id = match get_cookie(req, REQUEST_ID_HEADER_COOKIE) {
            Some(r) => r,
            _ => return Err(BotdError::NoRequestIdInCookie)
        };
        let endpoint = BotdEndpoint::new(config, "/results");
        let query = format!("header&token={}&id={}", config.token.to_owned(), request_id);
        log::debug!("[botd] request_id = {}, query: ?{}", request_id, query);
        let botd_resp = Request::get(config.botd_url.to_owned())
            .with_path(endpoint.path.as_str())
            .with_query_str(query)
            .send(BOTD_BACKEND_NAME);
        let botd_resp = match botd_resp {
            Ok(r) => r,
            Err(e) => return Err(BotdError::SendError(String::from(e.backend_name())))
        };
        if let Err(err) = check_resp(&botd_resp) { return Err(err) }
        transfer_headers(req, &botd_resp);
        Ok(BotDetector{ request_id })
    }
}