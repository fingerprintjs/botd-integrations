export const BOTD_DEFAULT_URL = 'https://botd.fpapi.io'
export const BOTD_DEFAULT_PATH = '/api/v1/'
export const BOTD_RESULT_PATH = '/api/v1/results'
export const BOTD_LIGHT_PATH = '/api/v1/light'

export const enum Status {
  PROCESSED = 'processed',
  ERROR = 'error',
}

export const GET = 'GET'
export const POST = 'POST'

export const REQUEST_ID_HEADER = 'botd-request-id'
export const ERROR_DESCRIPTION_HEADER = 'botd-error-description'
export const REQUEST_STATUS_HEADER = 'botd-request-status'

export const AUTO_TOOL_STATUS_HEADER = 'botd-automation-tool-status'
export const AUTO_TOOL_PROB_HEADER = 'botd-automation-tool-prob'
export const AUTO_TOOL_TYPE_HEADER = 'botd-automation-tool-type'

export const SEARCH_BOT_STATUS_HEADER = 'botd-search-bot-status'
export const SEARCH_BOT_PROB_HEADER = 'botd-search-bot-prob'
export const SEARCH_BOT_TYPE_HEADER = 'botd-search-bot-type'

export const BROWSER_SPOOFING_STATUS_HEADER = 'botd-browser-spoofing-status'
export const BROWSER_SPOOFING_PROB_HEADER = 'botd-browser-spoofing-prob'
export const BROWSER_SPOOFING_TYPE_HEADER = 'botd-browser-spoofing-type'

export const VM_STATUS_HEADER = 'botd-vm-status'
export const VM_PROB_HEADER = 'botd-vm-prob'
export const VM_TYPE_HEADER = 'botd-vm-type'

export const RESULT_HEADERS = [
  REQUEST_ID_HEADER,
  REQUEST_STATUS_HEADER,
  AUTO_TOOL_STATUS_HEADER,
  AUTO_TOOL_PROB_HEADER,
  AUTO_TOOL_TYPE_HEADER,
  VM_STATUS_HEADER,
  VM_PROB_HEADER,
  VM_TYPE_HEADER,
  SEARCH_BOT_STATUS_HEADER,
  SEARCH_BOT_PROB_HEADER,
  SEARCH_BOT_TYPE_HEADER,
  BROWSER_SPOOFING_STATUS_HEADER,
  BROWSER_SPOOFING_PROB_HEADER,
  BROWSER_SPOOFING_TYPE_HEADER,
]

export const LIGHT_RESULT_HEADERS = [
  REQUEST_ID_HEADER,
  REQUEST_STATUS_HEADER,
  AUTO_TOOL_STATUS_HEADER,
  AUTO_TOOL_PROB_HEADER,
  AUTO_TOOL_TYPE_HEADER,
]

export const SEC_FETCH_DEST_HEADER = 'sec-fetch-dest'
export const STATIC_SEC_FETCH_DEST = ['font', 'script', 'image', 'style', 'video', 'manifest', 'object'] // TODO: add all static types
export const STATIC_PATH_ENDINGS = ['.css', '.js', '.jpg', '.png', '.svg', '.jpeg', '.woff2'] // TODO: add all static types

export const COOKIE_NAME = 'botd-request-id'
export const COOKIE_HEADER = 'Cookie'
export const SET_COOKIE_HEADER = 'Set-Cookie'
