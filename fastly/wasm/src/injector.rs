use regex::Regex;
use crate::PATH_HASH;
use crate::config::Config;
use crate::error::BotdError;
use BotdError::{RegexSyntax, WrongHTML};

pub fn inject_script(html: &str, config: &Config) -> Result<String, BotdError> {
    log::debug!("[inject] Inject script with token: {}", config.token);
    let script_src: String;
    if config.agent_version == "latest" {
        script_src = format!("/{}/dist/", PATH_HASH);
    } else {
        if config.debug {
            script_src = format!("/{}/dist/version/{}", PATH_HASH, config.agent_version);
        } else {
            script_src = format!("/{}/dist/min/{}", PATH_HASH, config.agent_version);
        }
    }
    let script = format!("
    <script>function getResults(){{Botd.load({{token:\"{}\",endpoint:\"{}\",mode:\"integration\"}}).then(b=>{{return b.detect()}})}}</script>
    <script src=\"{}\" onload=\"getResults()\"></script>", config.token, PATH_HASH, script_src);
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