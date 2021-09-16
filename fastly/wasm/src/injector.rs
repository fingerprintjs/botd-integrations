use regex::Regex;
use crate::PATH_HASH;
use crate::config::Config;
use crate::error::BotdError;

pub fn inject_script(html: &str, config: &Config) -> Result<String, BotdError> {
    log::debug!("[inject_script] Inject script with token: {}", config.token);
    let script = format!("
    <script> function getResults() {{ Botd.load({{ token: \"{}\", endpoint: \"{}\", isIntegration: true}}).then( b => {{ return b.detect() }} ) }} </script>
    <script src=\"/{}/dist\" onload=\"getResults()\"></script>", config.token, PATH_HASH, PATH_HASH);
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