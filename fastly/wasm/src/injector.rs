use regex::Regex;
use crate::PATH_HASH;
use crate::config::Config;
use crate::error::BotdError;
use BotdError::{RegexSyntax, WrongHTML};

pub fn inject_script(html: &str, config: &Config) -> Result<String, BotdError> {
    log::debug!("[inject] Inject script with token: {}", config.token);
    let min = if config.debug { String::new() } else { String::from(".min") };
    let script_src = format!("/{}/dist/v{}/esm{}.js", PATH_HASH, config.agent_version, min);
    let script = format!("
    <script>
        function getResults() {{
            import(\'{}\')
            .then( Botd => Botd.load({{
                token:\'{}\',
                endpoint:\'{}\',
                mode:\'integration\'
            }}))
            .then( detector => detector.detect()) }}
        getResults()
    </script>", script_src, config.token, PATH_HASH);
    let mut result = html.to_owned();
    let re = r"(<head.*>)";
    if let Ok(r) = Regex::new(re) {
        if let Some(m) = r.find(html) {
            result.insert_str(m.end(), script.as_str());
            return Ok(result);
        }
        return Err(WrongHTML);
    }
    Err(RegexSyntax(String::from(re)))
}