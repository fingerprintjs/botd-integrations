use regex::Regex;
use crate::config::Config;

pub fn inject_botd_script(html: Box<str>, config: &Config) -> String {
    log::debug!("[inject_botd_script] Inject script with token: {}, endpoint: {}", config.token, config.botd_endpoint);
    let script = format!("
    <script>
        async function getResults() {{
            const botdPromise = Botd.load({{
                token: \"{}\",
                mode: \"requestId\",
                endpoint: \"{}\",
            }})
        const botd = await botdPromise
        const result = await botd.detect()
        }}
    </script>
    <script src=\"https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0/dist/botd.min.js\" onload=\"getResults()\"></script>
    ", config.token, config.botd_endpoint);

    let mut injected_html = String::from(html);
    let script_index = Regex::new(r"(<head.*>)").unwrap().find(&*injected_html).unwrap().end();

    injected_html.insert_str(script_index, script.as_str());
    injected_html
}