use crate::config::{DEFAULT_BOTD_PATH, Config};

pub struct BotdEndpoint {
    pub(crate) path: String,
    pub(crate) url: String,
}

impl BotdEndpoint {
    pub fn new(config: &Config, endpoint: &str) -> BotdEndpoint {
        let mut endpoint = String::from(endpoint);
        if endpoint.starts_with('/') { endpoint.remove(0); }
        let path = format!("{}{}", DEFAULT_BOTD_PATH, endpoint);
        let url = format!("{}{}", config.botd_url, path);
        BotdEndpoint { path, url }
    }
}