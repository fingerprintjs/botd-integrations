use regex::Regex;
use crate::config::Config;


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
    let debug_url_replacement = match config.debug_botd_url.to_owned() {
        Some(e) => format!("endpoint: \"{}\",", e),
        _ => String::from("")
    };

    log::debug!("[inject_script] Inject script with token: {}", config.token);
    let script = format!("
    <script>
        async function getResults() {{
            const botdPromise = Botd.load({{ token: \"{}\", mode: \"requestId\", {}}})
            const botd = await botdPromise
            const result = await botd.detect()
        }}
    </script>
    <script src=\"https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0/dist/botd.min.js\" onload=\"getResults()\"></script>
    ", config.token, debug_url_replacement);

    let mut result = html.to_owned();

    let re = r"(<head.*>)";
    if let Ok(r) = Regex::new(re) {
        if let Some(m) = r.find(html) {
            let i = m.end();
            result.insert_str(i, script.as_str());
            return Ok(result);
        }
        return Err(InjectorError::WrongHTML);
    }
    Err(InjectorError::RegexSyntax(format!("Can't create regex {}", re)))
}