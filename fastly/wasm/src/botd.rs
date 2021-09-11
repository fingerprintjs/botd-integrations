use fastly::Request;
use crate::config::{Config, BOTD_BACKEND_NAME};
use crate::COOKIE_NAME;
use crate::utils::get_cookie;
use crate::detector::{Detect, BLOB, check_resp, transfer_headers};
use crate::endpoint::BotdEndpoint;

pub struct BotDetector {
    request_id: String
}

impl Detect for BotDetector {
    fn make(req: &mut Request, config: &Config) -> Result<Self, String> {
        let request_id = match get_cookie(req, COOKIE_NAME) {
            Some(r) => r,
            _ => return Err(String::from("Can't get request id"))
        };
        let endpoint = BotdEndpoint::new("/results");
        let query = format!("header&token={}&id={}", config.token.to_owned(), request_id);

        log::debug!("[bot_detect] request_id = {}, query: ?{}", request_id, query);

        let botd_resp = Request::get(BLOB)
            .with_path(endpoint.path.as_str())
            .with_query_str(query)
            .send(BOTD_BACKEND_NAME);

        let botd_resp = match botd_resp {
            Ok(r) => r,
            Err(_) => return Err(String::from("Send error"))
        };

        if let Err(err) = check_resp(&botd_resp) {
            return Err(err)
        }
        transfer_headers(req, &botd_resp);

        Ok(BotDetector{ request_id })
    }

    fn get_request_id(&self) -> String {
        self.request_id.to_owned()
    }
}