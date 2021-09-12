use crate::config::DEFAULT_BOTD_PATH;

pub struct BotdEndpoint {
    pub(crate) path: String,
}

impl BotdEndpoint {
    pub fn new(endpoint: &str) -> BotdEndpoint {
        let mut endpoint = String::from(endpoint);
        if endpoint.starts_with('/') { endpoint.remove(0); }
        BotdEndpoint { path: format!("{}{}", DEFAULT_BOTD_PATH, endpoint) }
    }
}