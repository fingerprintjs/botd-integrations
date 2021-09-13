use regex::Regex;
use crate::config::Config;
use crate::error::BotdError;
use crate::endpoint::BotdEndpoint;

pub fn inject_script(html: &str, config: &Config) -> Result<String, BotdError> {
    log::debug!("[inject_script] Inject script with token: {}", config.token);
    let endpoint = BotdEndpoint::new(config, "/");
    let script = format!("
    <script> function getResults() {{ Botd.load({{ token: \"{}\", endpoint: \"{}\"}}).then( b => {{ return b.detect() }} ) }} </script>
    <script src=\"https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0/dist/botd.min.js\" onload=\"getResults()\"></script>", config.token, endpoint.url);
    let mut result = html.to_owned();
    let re = r"(<head.*>)";
    if let Ok(r) = Regex::new(re) {
        if let Some(m) = r.find(html) {
            result.insert_str(m.end(), script.as_str());
            return Ok(result);
        }
        return Err(BotdError::WrongHTML);
    }
    Err(BotdError::RegexSyntax(String::from(re)))
}