pub struct BotdEndpoint {
    // const_path: String,
    // user_endpoint: String,
    // fixed_endpoint: String,
    pub(crate) path: String,
}

impl BotdEndpoint {
    pub fn new(endpoint: &str) -> BotdEndpoint {
        let mut endpoint = String::from(endpoint);
        let const_path = String::from("/api/v1");
        if endpoint.starts_with('/') { endpoint.remove(0); }
        let path = format!("{}{}", const_path, endpoint);
        BotdEndpoint { path }
    }
}