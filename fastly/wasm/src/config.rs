use std::fmt;
use fastly::{Dictionary, Error};
use crate::utils::remove_trailing_slash;

const ENV_DEFAULT: &str = "Middleware";
const BOTD_URL: &str = "https://botd.fpapi.io/";

const CONFIG_DICT_NAME: &str = "botd_config";
const CONFIG_DISABLE: &str = "disable";
const CONFIG_ENV: &str = "env";
const CONFIG_TOKEN: &str = "token";
const CONFIG_BOTD_URL: &str = "botd_url";
const CONFIG_APP_URL: &str = "origin_url";

const TRUE: &str = "true";
const FALSE: &str = "false";

pub struct Config {
    pub env: String,
    pub token: String,
    pub botd_url: String,
    pub origin_url: String,
    pub disabled: bool
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let dictionary = Dictionary::open(CONFIG_DICT_NAME);

        let env = dictionary.get(CONFIG_ENV).unwrap_or(String::from(ENV_DEFAULT));
        let mut botd_url = dictionary.get(CONFIG_BOTD_URL).unwrap_or(String::from(BOTD_URL));
        let is_disabled_string = dictionary.get(CONFIG_DISABLE).unwrap_or(String::from(FALSE));
        let disabled = is_disabled_string == TRUE;

        let err_msg = format!("[Compute@Edge:BotdError] Can't get botd token from {} dictionary by key {}", CONFIG_DICT_NAME, CONFIG_TOKEN);
        let token = match dictionary.get(CONFIG_TOKEN).ok_or(err_msg) {
            Ok(t) => t,
            Err(e) => return Err(Error::msg(e))
        };

        let err_msg = format!("[Compute@Edge:BotdError] Can't get application backend URL from {} dictionary by key {}", CONFIG_DICT_NAME, CONFIG_APP_URL);
        let mut origin_url = match dictionary.get(CONFIG_APP_URL).ok_or(err_msg) {
            Ok(t) => t,
            Err(e) => return Err(Error::msg(e))
        };
        remove_trailing_slash(&mut botd_url);
        remove_trailing_slash(&mut origin_url);

        Ok(Config{
            env,
            token,
            botd_url,
            origin_url,
            disabled
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Botd data from Compute@Edge dictionary botd_config:\n\
            - Environment: {},\n\
            - Token: {},\n\
            - App Backend: {},\n\
            - Botd Backend URL: {},\n\
            - Is Botd disabled: {}",
                     self.env,
                     self.token,
                     self.origin_url,
                     self.botd_url,
                     self.disabled
            )
    }
}