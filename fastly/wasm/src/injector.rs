use regex::Regex;
use crate::PATH_HASH;
use crate::config::Config;
use crate::error::BotdError;
use BotdError::{RegexSyntax, WrongHTML};

pub fn inject_script(html: &str, config: &Config) -> Result<String, BotdError> {
    log::debug!("[inject] Inject script with token: {}", config.token);
    let script = format!("
    <script>function getResults(){{Botd.load({{token:\"{}\",endpoint:\"{}\",isIntegration:true}}).then(b=>{{return b.detect()}})}}</script>
    <script src=\"/{}/dist/npm/@fpjs-incubator/botd-agent@0.1.18-beta.1/dist/botd.js\" onload=\"getResults()\"></script>", config.token, PATH_HASH, PATH_HASH);
    let mut result = html.to_owned();
    let re = r"(<head.*>)";
    if let Ok(r) = Regex::new(re) {
        if let Some(m) = r.find(html) {
            result.insert_str(m.end(), script.as_str());
            return Ok(result)
        }
        return Err(WrongHTML)
    }
    Err(RegexSyntax(String::from(re)))
}