use crate::config::DEFAULT_BOTD_PATH;

pub struct BotdEndpoint {
    const_path: String,
    user_endpoint: String,
    fixed_endpoint: String,
    pub(crate) path: String,
}

impl BotdEndpoint {
    pub fn new(endpoint: &str) -> BotdEndpoint {
        let user_endpoint = String::from(endpoint);
        let const_path = String::from(DEFAULT_BOTD_PATH);
        let mut fixed_endpoint = user_endpoint.to_owned();
        if fixed_endpoint.starts_with('/') { fixed_endpoint.remove(0) }
        let path = format!("{}{}", const_path.to_owned(), fixed_endpoint.to_owned());
        BotdEndpoint { const_path, user_endpoint, fixed_endpoint, path }
    }
}