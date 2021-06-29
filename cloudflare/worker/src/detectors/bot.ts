import { DetectResultError, DetectResultItem, getRequestID } from '../utils'
import Config from '../config'
import {
  BOTD_RESULT_PATH, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER,
  AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_TYPE_HEADER,
  BROWSER_SPOOFING_PROB_HEADER, BROWSER_SPOOFING_STATUS_HEADER, BROWSER_SPOOFING_TYPE_HEADER,
  SEARCH_BOT_PROB_HEADER, SEARCH_BOT_STATUS_HEADER, SEARCH_BOT_TYPE_HEADER,
  VM_PROB_HEADER, VM_STATUS_HEADER, VM_TYPE_HEADER, GET, ERROR_DESCRIPTION_HEADER, Status,
} from '../constants'

interface BotDetectResultProcessed {
  status: Status.PROCESSED
  requestID: string
  autoTool: DetectResultItem
  searchBot: DetectResultItem
  vm: DetectResultItem
  browserSpoofing: DetectResultItem
}

type BotDetectResult = BotDetectResultProcessed | DetectResultError

function botResultFromHeaders(headers: Headers): BotDetectResult {
  const status = headers.get(REQUEST_STATUS_HEADER) || ''

  if (status === Status.ERROR) {
    const error = headers.get(ERROR_DESCRIPTION_HEADER) || ''
    console.error(`[botResultFromHeaders] Handled error from Botd backend: ${error}`)
    return { status, error }

  } else if (status === Status.PROCESSED) {
    const requestID = headers.get(REQUEST_ID_HEADER) || ''

    const autoToolStatus = headers.get(AUTO_TOOL_STATUS_HEADER) || ''
    const autoToolProbability = headers.get(AUTO_TOOL_PROB_HEADER) || ''
    const autoToolType = headers.get(AUTO_TOOL_TYPE_HEADER) || ''

    const vmStatus = headers.get(VM_STATUS_HEADER) || ''
    const vmProbability = headers.get(VM_PROB_HEADER) || ''
    const vmType = headers.get(VM_TYPE_HEADER) || ''

    const searchBotStatus = headers.get(SEARCH_BOT_STATUS_HEADER) || ''
    const searchBotProbability = headers.get(SEARCH_BOT_PROB_HEADER) || ''
    const searchBotType = headers.get(SEARCH_BOT_TYPE_HEADER) || ''

    const browserSpoofingStatus = headers.get(BROWSER_SPOOFING_STATUS_HEADER) || ''
    const browserSpoofingProbability = headers.get(BROWSER_SPOOFING_PROB_HEADER) || ''
    const browserSpoofingType = headers.get(BROWSER_SPOOFING_TYPE_HEADER) || ''

    return {
      requestID,
      status,
      autoTool: {
        status: autoToolStatus,
        prob: parseFloat(autoToolProbability),
        type: autoToolType,
      },
      vm: {
        status: vmStatus,
        prob: parseFloat(vmProbability),
        type: vmType,
      },
      searchBot: {
        status: searchBotStatus,
        prob: parseFloat(searchBotProbability),
        type: searchBotType,
      },
      browserSpoofing: {
        status: browserSpoofingStatus,
        prob: parseFloat(browserSpoofingProbability),
        type: browserSpoofingType,
      },
    }
  } else
    throw Error(`Unknown status from bot detection server: ${status}`)
}

async function requestBotDetect(url: string): Promise<Response> {
  try {
    const botRequest = new Request(url.toString(), { method: GET })
    return await fetch(botRequest)
  } catch (e) {
    console.error(`[requestLightDetect] Error handled: ${e.message}`)
    throw Error(`Error during bot detection: ${e.message}`)
  }
}

export function setBotDetectHeaders(r: Response | Request, results: BotDetectResult): void {
  r.headers.append(REQUEST_STATUS_HEADER, results.status)

  if (results.status === Status.ERROR) {
    r.headers.append(ERROR_DESCRIPTION_HEADER, results.error)
  } else {
    r.headers.append(REQUEST_ID_HEADER, results.requestID)
    r.headers.append(AUTO_TOOL_STATUS_HEADER, results.autoTool.status)
    r.headers.append(AUTO_TOOL_PROB_HEADER, results.autoTool.prob.toFixed(2))
    r.headers.append(AUTO_TOOL_TYPE_HEADER, results.autoTool.type)
    r.headers.append(VM_STATUS_HEADER, results.vm.status)
    r.headers.append(VM_PROB_HEADER, results.vm.prob.toFixed(2))
    r.headers.append(VM_TYPE_HEADER, results.vm.type)
    r.headers.append(BROWSER_SPOOFING_STATUS_HEADER, results.browserSpoofing.status)
    r.headers.append(BROWSER_SPOOFING_PROB_HEADER, results.browserSpoofing.prob.toFixed(2))
    r.headers.append(BROWSER_SPOOFING_TYPE_HEADER, results.browserSpoofing.type)
    r.headers.append(SEARCH_BOT_STATUS_HEADER, results.searchBot.status)
    r.headers.append(SEARCH_BOT_PROB_HEADER, results.searchBot.prob.toFixed(2))
    r.headers.append(SEARCH_BOT_TYPE_HEADER, results.searchBot.type)
  }
}

export async function makeBotDetect(request: Request, config: Config): Promise<BotDetectResult> {
  const requestID = getRequestID(request)
  const url = `${config.botdURL}${BOTD_RESULT_PATH}?header&token=${config.token}&id=${requestID}`
  const botdResponse = await requestBotDetect(url)
  const botdResult = botResultFromHeaders(botdResponse.headers)

  if (botdResult.status === Status.ERROR) {
    console.log(`[makeBotDetect] Bot Detect Result - Status: ${botdResult.status}, Error message: ${botdResult.error}`)
  } else {
    console.log(`[makeBotDetect] Bot Detect Result - Status: ${botdResult.status}, Request ID: ${botdResult.requestID}`)
    console.log(`[makeBotDetect] Automation Tool - Status: ${botdResult.autoTool.status}, Probability: ${botdResult.autoTool.prob}, Type: ${botdResult.autoTool.type}`)
    console.log(`[makeBotDetect] Search Bot - Status: ${botdResult.searchBot.status}, Probability: ${botdResult.searchBot.prob}, Type: ${botdResult.searchBot.type}`)
    console.log(`[makeBotDetect] Virtual Machine - Status: ${botdResult.vm.status}, Probability: ${botdResult.vm.prob}, Type: ${botdResult.vm.type}`)
    console.log(`[makeBotDetect] Browser Spoofing - Status: ${botdResult.browserSpoofing.status}, Probability: ${botdResult.browserSpoofing.prob}, Type: ${botdResult.browserSpoofing.type}`)
  }
  return botdResult
}
