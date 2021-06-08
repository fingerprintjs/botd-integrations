use fastly::{Dictionary, Error};
use crate::constants::{BOTD_URL, ENV_DEFAULT};

pub struct Config {
    pub env: String,
    pub botd_token: String,
    pub botd_url: String,
    pub app_backend_url: String,
}

fn get_variable(name: &str, dictionary: &Dictionary) -> String {
    let option = dictionary.get(name);
    if option.is_none() {
        let msg = name.to_owned() + " cannot be extracted from config";
        log::error!("{}", msg.to_owned());
        panic!(msg)
    }
    return option.unwrap();
}

fn remove_last_slash(src: String) -> String {
    if src.chars().last() == Some('/') {
        return String::from(&src[..src.len() - 1]);
    }
    return src
}

fn get_variable_or_default(name: &str, default: &str, dictionary: &Dictionary) -> String {
    let option = dictionary.get(name);
    if option.is_none() {
        return default.to_string()
    }
    return option.unwrap();
}

pub fn read_config() -> Result<Config, Error> {
    let dictionary = Dictionary::open("config");
    let env = get_variable_or_default("env", ENV_DEFAULT, &dictionary);
    let botd_token = get_variable("botd_token", &dictionary);
    let botd_url = remove_last_slash(get_variable_or_default("botd_url", BOTD_URL, &dictionary));
    let app_backend_url = remove_last_slash(get_variable("app_backend_url", &dictionary));
    return Result::Ok(Config{env, botd_token, botd_url, app_backend_url})
}