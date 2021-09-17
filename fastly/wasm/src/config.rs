use crate::error::BotdError;
use fastly::Dictionary;
use BotdError::Disabled;

/// This should match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub const APP_BACKEND_NAME: &str = "backend";
pub const BOTD_BACKEND_NAME: &str = "botd";
pub const CDN_BACKEND_NAME: &str = "cdn";

pub struct Config {
    pub token: String,
    pub log_endpoint_name: String,
    // Needs for CORS
    pub app_host: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, BotdError> {
        const DEFAULT_LOG_ENDPOINT: &str = "local";
        const CONFIG_DICT_NAME: &str = "botd_config";
        const CONFIG_TOKEN: &str = "token";
        const CONFIG_LOG_ENDPOINT: &str = "log_endpoint";
        const CONFIG_DISABLE: &str = "disable";
        const CONFIG_APP_HOST: &str = "app_host";

        let dictionary = Dictionary::open(CONFIG_DICT_NAME);
        let token = match dictionary.get(CONFIG_TOKEN) {
            Some(t) => t,
            _ => return Err(BotdError::NoTokenInConfig)
        };
        let app_host = dictionary.get(CONFIG_APP_HOST);
        let log_endpoint_name_default = || String::from(DEFAULT_LOG_ENDPOINT);
        let log_endpoint_name = dictionary.get(CONFIG_LOG_ENDPOINT).unwrap_or_else(log_endpoint_name_default);
        if let Some(d) = dictionary.get(CONFIG_DISABLE) {
            if d == true.to_string() { return Err(Disabled); }
        }
        Ok(Config { token, log_endpoint_name, app_host })
    }
}