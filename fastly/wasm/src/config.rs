use std::fmt;
use fastly::{Dictionary};

/// This should match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub const BOTD_BACKEND_NAME: &str = "Botd";
pub const APP_BACKEND_NAME: &str = "Backend";

const DEFAULT_LOG_ENDPOINT_NAME: &str = "Local";
const CONFIG_DICT_NAME: &str = "botd_config";
const CONFIG_DISABLE: &str = "disable";
const CONFIG_LOG_ENDPOINT_NAME: &str = "log_endpoint_name";
const CONFIG_TOKEN: &str = "token";
const CONFIG_BOTD_URL: &str = "botd_url";
const CONFIG_APP_HOST: &str = "app_host";

pub struct Config {
    pub token: String,
    pub log_endpoint_name: String,
    // Needs for debug purpose, should be used only in injector
    pub debug_botd_url: Option<String>,
    // Needs for CORS
    pub app_host: Option<String>,
    pub botd_backend_name: String,
    pub app_backend_name: String,
}

/// An error that occurred during creation botd config
pub enum ConfigError {
    /// Can't extract botd token.
    NoToken,
    /// Passed HTML string doesn't contain <head> tag
    Disabled,
}

impl ToString for ConfigError {
    fn to_string(&self) -> String {
        return match self {
            ConfigError::NoToken => format!("Can't get botd token from {} dictionary by key {}", CONFIG_DICT_NAME, CONFIG_TOKEN),
            ConfigError::Disabled => String::from("Bot detection disabled")
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let dictionary = Dictionary::open(CONFIG_DICT_NAME);

        let token = match dictionary.get(CONFIG_TOKEN) {
            Some(t) => t,
            _ => {
                return Err(ConfigError::NoToken)
            }
        };

        let debug_botd_url = dictionary.get(CONFIG_BOTD_URL);
        let app_host = dictionary.get(CONFIG_APP_HOST);

        let log_endpoint_name_default = || String::from(DEFAULT_LOG_ENDPOINT_NAME);
        let log_endpoint_name = dictionary.get(CONFIG_LOG_ENDPOINT_NAME).unwrap_or_else(log_endpoint_name_default);

        let botd_backend_name = String::from(BOTD_BACKEND_NAME);
        let app_backend_name = String::from(APP_BACKEND_NAME);

        if let Some(d) = dictionary.get(CONFIG_DISABLE) {
            if d == true.to_string() { return Err(ConfigError::Disabled) }
        }

        Ok(Config{
            token,
            log_endpoint_name,
            debug_botd_url,
            app_host,
            botd_backend_name,
            app_backend_name
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Botd data from Compute@Edge dictionary botd_config:\n\
            - Environment: {},\n\
            - Token: {},\n\
            - App Backend: {},\n\
            - Botd Backend: {}",
                   self.log_endpoint_name,
                   self.token,
                   self.app_backend_name,
                   self.botd_backend_name
            )
    }
}