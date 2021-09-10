use fastly::Request;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::COOKIE_NAME;
use crate::utils::get_cookie;
use crate::detector::{Detect, BLOB};
use crate::endpoint::BotdEndpoint;

pub struct BotDetector {
    request_id: String
}

impl Detect for BotDetector {
    fn make(req: &Request, config: &Config) -> Result<Self, &str> {
        let request_id = match get_cookie(req, COOKIE_NAME) {
            Some(r) => r,
            _ => return Err("Can't get request id")
        };
        let endpoint = BotdEndpoint::new("/results");
        let query = format!("header&token={}&id={}", config.token.to_owned(), request_id.to_owned());

        log::debug!("[bot_detect] request_id = {}, query: ?{}", request_id.to_owned(), query.to_owned());

        let botd_resp = Request::get(BLOB)
            .with_path(endpoint.path.as_str())
            .with_query_str(query)
            .send(BOTD_BACKEND_NAME).ok();

        Ok(BotDetector{ request_id })
    }

    fn get_request_id(&self) -> String {
        Ok(self.request_id.to_owned())
    }
}