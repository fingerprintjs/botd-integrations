use regex::Regex;
use crate::config::Config;
use crate::constants::BOTD_DEFAULT_PATH;

fn get_script(token: String, endpoint: String) -> String {
    const SCRIPT_BODY_BEGIN: &str = r#"
    <script>
        async function getResults() {
            const botdPromise = Botd.load({
            token: ""#;
    const SCRIPT_BODY_MIDDLE: &str = r#"",
            mode: "requestId",
            endpoint: ""#;
    const SCRIPT_BODY_END: &str = r#"",
        })
        const botd = await botdPromise
        const result = await botd.get({isPlayground: true})
        }
    </script>
    <script src="https://unpkg.com/@fpjs-incubator/botd-agent@0/dist/botd.umd.min.js" onload="getResults()"></script>
    "#;
    return format!("{}{}{}{}{}", SCRIPT_BODY_BEGIN, token, SCRIPT_BODY_MIDDLE, endpoint, SCRIPT_BODY_END)
}

pub fn add_bot_detection_script(html: Box<str>, config: &Config) -> String {
    let mut injected_html = String::from(html);

    let endpoint = format!("{}{}", config.botd_url, BOTD_DEFAULT_PATH);
    let script = get_script(config.botd_token.to_owned(), endpoint.to_owned());

    log::debug!("[add_bot_detection_script] token: {}, endpoint: {}", config.botd_token, endpoint);

    let head_regex = Regex::new(r"(<head.*>)").unwrap();
    let script_index = head_regex.find(&*injected_html).unwrap().end();

    injected_html.insert_str(script_index, &script);

    return injected_html;
}