use regex::Regex;
use crate::config::{Config, DEFAULT_BOTD_PATH};


/// An error that occurred during injecting botd script
pub enum InjectorError {
    /// A regex syntax error.
    RegexSyntax(String),
    /// Passed HTML string doesn't contain <head> tag
    WrongHTML,
}

impl ToString for InjectorError {
    fn to_string(&self) -> String {
        match self {
            InjectorError::RegexSyntax(s) => s.to_owned(),
            InjectorError::WrongHTML => String::from("Can't find head tag in response body.")
        }
    }
}

pub fn inject_script(html: &str, config: &Config) -> Result<String, InjectorError> {
    log::debug!("[inject_script] Inject script with token: {}", config.token);
    let endpoint = format!("{}{}", config.botd_url, DEFAULT_BOTD_PATH);
    let script = format!("
    <script>
        async function getResults() {{
            const botdPromise = Botd.load({{ token: \"{}\", mode: \"requestId\", endpoint: \"{}\"}})
            const botd = await botdPromise
            const result = await botd.detect()
        }}
    </script>
    <script src=\"https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0/dist/botd.min.js\" onload=\"getResults()\"></script>
    ", config.token, endpoint);
    let mut result = html.to_owned();
    let re = r"(<head.*>)";
    if let Ok(r) = Regex::new(re) {
        if let Some(m) = r.find(html) {
            result.insert_str(m.end(), script.as_str());
            return Ok(result);
        }
        return Err(InjectorError::WrongHTML);
    }
    Err(InjectorError::RegexSyntax(format!("Can't create regex {}", re)))
}