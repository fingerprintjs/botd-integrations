use fastly::{Dictionary, Error};

pub struct Config {
    pub env: String,
    pub botd_token: String,
    pub botd_results_url: String,
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

pub fn read_config() -> Result<Config, Error> {
    let dictionary = Dictionary::open("config");
    let env = get_variable("env", &dictionary);
    let botd_token = get_variable("botd_token", &dictionary);
    let botd_results_url = get_variable("botd_results_url", &dictionary);
    let app_backend_url = get_variable("app_backend_url", &dictionary);
    return Result::Ok(Config{env, botd_token, botd_results_url, app_backend_url})
}