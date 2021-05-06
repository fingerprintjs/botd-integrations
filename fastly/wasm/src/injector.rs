use regex::Regex;

const SCRIPT_CONNECT: &str = r#"<script async src="https://unpkg.com/@fpjs-incubator/botd-agent@0/dist/botd.umd.min.js" onload="getResults()"></script>"#;

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
    </script>"#;

pub fn add_bot_detection_script(html: Box<str>, botd_token: &str) -> String {
    let mut injected_html = String::from(html);

    // Insert script in <head>
    let head_close_regex = Regex::new(r"(</head.*>)").unwrap();
    let connect_index = head_close_regex.find(&*injected_html).unwrap().start();
    injected_html.insert_str(connect_index, SCRIPT_CONNECT);

    // Insert script which sends signals to bot detection API
    let body_open_regex = Regex::new(r"(<body.*>)").unwrap();
    let mut script_index = body_open_regex.find(&*injected_html).unwrap().end();
    injected_html.insert_str(script_index, SCRIPT_BODY_BEGIN);
    script_index += SCRIPT_BODY_BEGIN.len();
    injected_html.insert_str(script_index, botd_token);
    script_index += botd_token.len();
    injected_html.insert_str(script_index, SCRIPT_BODY_END);

    return injected_html;
}