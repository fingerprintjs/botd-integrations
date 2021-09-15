const DEFAULT_BOTD_PATH: &str = "/api/v1/";

pub struct BotdEndpoint {
    pub(crate) path: String,
}

impl BotdEndpoint {
    pub fn new(endpoint: &str) -> BotdEndpoint {
        let mut endpoint = String::from(endpoint);
        if endpoint.starts_with('/') { endpoint.remove(0); }
        let path = format!("{}{}", DEFAULT_BOTD_PATH, endpoint);
        BotdEndpoint { path }
    }
}