use regex::Regex;
use crate::config::Config;

const SCRIPT_BODY_BEGIN: &str = r#"
    <script>
        async function getResults() {
            const botdPromise = FPJSBotDetect.load({
            token: ""#;
const SCRIPT_BODY_END: &str = r#"",
            async: true,
        })
        const botd = await botdPromise
        const result = await botd.get({isPlayground: true})
        }
    </script>
    <script src="https://unpkg.com/@fpjs-incubator/botd-agent@0/dist/botd.umd.min.js" onload="getResults()"></script>
    "#;

pub fn add_bot_detection_script(html: Box<str>, config: &Config) -> String {
    let mut injected_html = String::from(html);

    let head_regex = Regex::new(r"(<head.*>)").unwrap();
    let mut script_index = head_regex.find(&*injected_html).unwrap().end();

    injected_html.insert_str(script_index, SCRIPT_BODY_BEGIN);
    script_index += SCRIPT_BODY_BEGIN.len();

    injected_html.insert_str(script_index, &*config.botd_token);
    script_index += config.botd_token.len();

    injected_html.insert_str(script_index, SCRIPT_BODY_END);

    return injected_html;
}