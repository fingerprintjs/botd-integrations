use crate::error::BotdError;
use fastly::{Dictionary, Request};
use BotdError::{Disabled, NoTokenInConfig};
use log::LevelFilter::{Debug, Info};
use crate::utils::get_ip;

/// This should match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub const APP_BACKEND_NAME: &str = "backend";
pub const BOTD_BACKEND_NAME: &str = "botd";
pub const CDN_BACKEND_NAME: &str = "cdn";

pub struct Config {
    pub token: String,
    pub ip: String,
}

impl Config {
    pub fn new(req: &Request) -> Result<Self, BotdError> {
        const DEFAULT_LOG_ENDPOINT: &str = "default";
        const CONFIG_DICT_NAME: &str = "botd_config";
        const CONFIG_TOKEN: &str = "token";
        const CONFIG_LOG_ENDPOINT: &str = "log_endpoint";
        const CONFIG_DISABLE: &str = "disable";
        const CONFIG_DEBUG: &str = "debug";

        let dictionary = Dictionary::open(CONFIG_DICT_NAME);

        let debug_default = || false.to_string();
        let debug = dictionary.get(CONFIG_DEBUG).unwrap_or_else(debug_default);
        let need_debug = true.to_string() == debug;

        let log_endpoint_name_default = || String::from(DEFAULT_LOG_ENDPOINT);
        let log_endpoint_name = dictionary.get(CONFIG_LOG_ENDPOINT).unwrap_or_else(log_endpoint_name_default);

        if need_debug {
            log_fastly::init_simple(log_endpoint_name, Debug);
        } else {
            log_fastly::init_simple(log_endpoint_name, Info);
        }

        let ip = get_ip(req);

        if let Some(d) = dictionary.get(CONFIG_DISABLE) {
            if d == true.to_string() { return Err(Disabled); }
        }
        let token = match dictionary.get(CONFIG_TOKEN) {
            Some(t) => t,
            _ => return Err(NoTokenInConfig)
        };

        Ok(Config { token, ip })
    }
}