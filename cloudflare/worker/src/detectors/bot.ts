import { Config, DetectResultItem, getRequestID } from '../utils'
import {
  BOTD_RESULT_PATH, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER,
  AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_TYPE_HEADER,
  BROWSER_SPOOFING_PROB_HEADER, BROWSER_SPOOFING_STATUS_HEADER, BROWSER_SPOOFING_TYPE_HEADER,
  SEARCH_BOT_PROB_HEADER, SEARCH_BOT_STATUS_HEADER, SEARCH_BOT_TYPE_HEADER,
  VM_PROB_HEADER, VM_STATUS_HEADER, VM_TYPE_HEADER,
} from '../constants'

export interface BotDetectResult {
  requestID: string
  status: string
  autoTool: DetectResultItem
  searchBot: DetectResultItem
  vm: DetectResultItem
  browserSpoofing: DetectResultItem
}

export async function makeBotDetect(request: Request, config: Config): Promise<BotDetectResult> {
  const requestID = getRequestID(request)
  const url = `${config.botdURL}${BOTD_RESULT_PATH}?header&token=${config.token}&id=${requestID}`
  console.log(url)
  const lightResponse = await requestBotDetect(url)
  return botResultFromHeaders(lightResponse.headers)
}

function botResultFromHeaders(headers: Headers): BotDetectResult {
  const request_status = headers.get(REQUEST_STATUS_HEADER) || ""
  const request_id = headers.get(REQUEST_ID_HEADER) || ""

  const auto_tool_status = headers.get(AUTO_TOOL_STATUS_HEADER) || ""
  const auto_tool_probability = headers.get(AUTO_TOOL_PROB_HEADER) || ""
  const auto_tool_type = headers.get(AUTO_TOOL_TYPE_HEADER) || ""

  const vm_status = headers.get(VM_STATUS_HEADER) || ""
  const vm_probability = headers.get(VM_PROB_HEADER) || ""
  const vm_type = headers.get(VM_TYPE_HEADER) || ""

  const search_bot_status = headers.get(SEARCH_BOT_STATUS_HEADER) || ""
  const search_bot_probability = headers.get(SEARCH_BOT_PROB_HEADER) || ""
  const search_bot_type = headers.get(SEARCH_BOT_TYPE_HEADER) || ""

  const browser_spoofing_status = headers.get(BROWSER_SPOOFING_STATUS_HEADER) || ""
  const browser_spoofing_probability = headers.get(BROWSER_SPOOFING_PROB_HEADER) || ""
  const browser_spoofing_type = headers.get(BROWSER_SPOOFING_TYPE_HEADER) || ""

  return {
    requestID: request_id,
    status: request_status,
    autoTool: {
      status: auto_tool_status,
      prob: parseFloat(auto_tool_probability),
      type: auto_tool_type
    },
    vm: {
      status: vm_status,
      prob: parseFloat(vm_probability),
      type: vm_type
    },
    searchBot: {
      status: search_bot_status,
      prob: parseFloat(search_bot_probability),
      type: search_bot_type
    },
    browserSpoofing: {
      status: browser_spoofing_status,
      prob: parseFloat(browser_spoofing_probability),
      type: browser_spoofing_type
    }
  }
}

async function requestBotDetect(url: string): Promise<Response> {
  const botRequestInit = {
    method: "GET"
  }
  const botRequest = new Request(url.toString(), botRequestInit)

  try {
    return await fetch(botRequest);
  } catch (e) {
    console.error(e.message)
    return new Response(JSON.stringify({ error: e.message }), { status: 500 });
  }
}
