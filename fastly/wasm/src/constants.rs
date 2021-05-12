pub const LOGGER: &str = "loggly";

pub const BOTD_BACKEND: &str = "Botd";
pub const APP_BACKEND: &str = "Backend";

pub const FAILED_STR: &str = "failed";
pub const OK_STR: &str = "ok";

pub const REQUEST_ID_HEADER: &str = "fpjs-request-id";
pub const REQUEST_STATUS_HEADER: &str = "fpjs-request-status";

pub const BOT_STATUS_HEADER: &str = "fpjs-bot-status";
pub const BOT_PROB_HEADER: &str = "fpjs-bot-prob";
pub const BOT_TYPE_HEADER: &str = "fpjs-bot-type";

pub const SEARCH_BOT_STATUS_HEADER: &str = "fpjs-search-bot-status";
pub const SEARCH_BOT_PROB_HEADER: &str = "fpjs-search-bot-prob";
pub const SEARCH_BOT_TYPE_HEADER: &str = "fpjs-search-bot-type";

pub const BROWSER_SPOOFING_STATUS_HEADER: &str = "fpjs-browser-spoofing-status";
pub const BROWSER_SPOOFING_PROB_HEADER: &str = "fpjs-browser-spoofing-prob";

pub const VM_STATUS_HEADER: &str = "fpjs-vm-status";
pub const VM_PROB_HEADER: &str = "fpjs-vm-prob";
pub const VM_TYPE_HEADER: &str = "fpjs-vm-type";

pub const SEC_FETCH_DEST_HEADER: &str = "sec-fetch-dest";
pub const STATIC_SEC_FETCH_DEST: [&'static str; 7] = ["font", "script", "image", "style", "video", "manifest", "object"]; // TODO: add all static types
pub const STATIC_PATH_ENDINGS: [&'static str; 7] = [".css", ".js", ".jpg", ".png", ".svg", ".jpeg", ".woff2"]; // TODO: add all static types

pub const COOKIE_NAME: &str = "botd-request-id=";
pub const COOKIE_HEADER: &str = "cookie";

pub const FORBIDDEN_BODY: &str = "{\"error\": {\"code\": 403, \"description\": \"Forbidden\"}}";