pub const BOTD_BACKEND: &str = "Stage";
pub const BOTD_URL: &str = "https://botd.fpapi.io/";
pub const BOTD_DEFAULT_PATH: &str = "/api/v1/";
pub const BOTD_RESULT_PATH: &str = "/api/v1/results";
pub const BOTD_LIGHT_PATH: &str = "/api/v1/light";

pub const ENV_DEFAULT: &str = "Middleware";
pub const APP_BACKEND: &str = "Backend";

pub const ERROR: &str = "error";
pub const PROCESSED: &str = "processed";

pub const REQUEST_ID_HEADER:     &str = "botd-request-id";
pub const REQUEST_STATUS_HEADER: &str = "botd-request-status";
pub const ERROR_DESCRIPTION:     &str = "botd-error-description";

pub const AUTO_TOOL_STATUS_HEADER: &str = "botd-automation-tool-status";
pub const AUTO_TOOL_PROB_HEADER:   &str = "botd-automation-tool-prob";
pub const AUTO_TOOL_TYPE_HEADER:   &str = "botd-automation-tool-type";

pub const SEARCH_BOT_STATUS_HEADER: &str = "botd-search-bot-status";
pub const SEARCH_BOT_PROB_HEADER:   &str = "botd-search-bot-prob";
pub const SEARCH_BOT_TYPE_HEADER:   &str = "botd-search-bot-type";

pub const BROWSER_SPOOFING_STATUS_HEADER: &str = "botd-browser-spoofing-status";
pub const BROWSER_SPOOFING_PROB_HEADER:   &str = "botd-browser-spoofing-prob";
pub const BROWSER_SPOOFING_TYPE_HEADER:   &str = "botd-browser-spoofing-type";

pub const VM_STATUS_HEADER: &str = "botd-vm-status";
pub const VM_PROB_HEADER:   &str = "botd-vm-prob";
pub const VM_TYPE_HEADER:   &str = "botd-vm-type";

pub const SEC_FETCH_DEST_HEADER: &str = "sec-fetch-dest";
pub const STATIC_SEC_FETCH_DEST: [&'static str; 7] = ["font", "script", "image", "style", "video", "manifest", "object"]; // TODO: add all static types
pub const STATIC_PATH_ENDINGS: [&'static str; 7] = [".css", ".js", ".jpg", ".png", ".svg", ".jpeg", ".woff2"]; // TODO: add all static types

pub const COOKIE_NAME: &str = "botd-request-id=";
pub const COOKIE_HEADER: &str = "cookie";
pub const SET_COOKIE_HEADER: &str = "set-cookie";