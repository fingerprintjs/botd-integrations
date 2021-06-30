import { getRequestID } from '../utils'
import Config from '../config'
import {
  BOTD_RESULT_PATH, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER,
  AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_TYPE_HEADER,
  BROWSER_SPOOFING_PROB_HEADER, BROWSER_SPOOFING_STATUS_HEADER, BROWSER_SPOOFING_TYPE_HEADER,
  SEARCH_BOT_PROB_HEADER, SEARCH_BOT_STATUS_HEADER, SEARCH_BOT_TYPE_HEADER,
  VM_PROB_HEADER, VM_STATUS_HEADER, VM_TYPE_HEADER, GET, ERROR_DESCRIPTION_HEADER, Status,
} from '../constants'

export function transferBotdHeaders(src: Response, dst: Request): void {
  const status = src.headers.get(REQUEST_STATUS_HEADER) || ''
  dst.headers.append(REQUEST_STATUS_HEADER, status)

  if (status === Status.ERROR) {
    const error = src.headers.get(ERROR_DESCRIPTION_HEADER) || ''
    dst.headers.append(ERROR_DESCRIPTION_HEADER, error)

    console.error(`[transferBotdHeaders] Handled error from Botd backend: ${error}`)

  } else if (status === Status.PROCESSED) {
    const requestID = src.headers.get(REQUEST_ID_HEADER) || ''
    const autoToolStatus = src.headers.get(AUTO_TOOL_STATUS_HEADER) || ''
    const autoToolProb = src.headers.get(AUTO_TOOL_PROB_HEADER) || ''
    const autoToolType = src.headers.get(AUTO_TOOL_TYPE_HEADER) || ''
    const vmStatus = src.headers.get(VM_STATUS_HEADER) || ''
    const vmProb = src.headers.get(VM_PROB_HEADER) || ''
    const vmType = src.headers.get(VM_TYPE_HEADER) || ''
    const searchBotStatus = src.headers.get(SEARCH_BOT_STATUS_HEADER) || ''
    const searchBotProb = src.headers.get(SEARCH_BOT_PROB_HEADER) || ''
    const searchBotType = src.headers.get(SEARCH_BOT_TYPE_HEADER) || ''
    const browserSpoofingStatus = src.headers.get(BROWSER_SPOOFING_STATUS_HEADER) || ''
    const browserSpoofingProb = src.headers.get(BROWSER_SPOOFING_PROB_HEADER) || ''
    const browserSpoofingType = src.headers.get(BROWSER_SPOOFING_TYPE_HEADER) || ''

    dst.headers.append(REQUEST_ID_HEADER, requestID)
    dst.headers.append(AUTO_TOOL_STATUS_HEADER, autoToolStatus)
    dst.headers.append(AUTO_TOOL_PROB_HEADER, autoToolProb)
    dst.headers.append(AUTO_TOOL_TYPE_HEADER, autoToolType)
    dst.headers.append(VM_STATUS_HEADER, vmStatus)
    dst.headers.append(VM_PROB_HEADER, vmProb)
    dst.headers.append(VM_TYPE_HEADER, vmType)
    dst.headers.append(SEARCH_BOT_STATUS_HEADER, searchBotStatus)
    dst.headers.append(SEARCH_BOT_PROB_HEADER, searchBotProb)
    dst.headers.append(SEARCH_BOT_TYPE_HEADER, searchBotType)
    dst.headers.append(BROWSER_SPOOFING_STATUS_HEADER, browserSpoofingStatus)
    dst.headers.append(BROWSER_SPOOFING_PROB_HEADER, browserSpoofingProb)
    dst.headers.append(BROWSER_SPOOFING_TYPE_HEADER, browserSpoofingType)

    console.log(`[transferBotdHeaders] Bot Detect Result - Status: ${status}, Request ID: ${requestID}`)
    console.log(`[transferBotdHeaders] Automation Tool - Status: ${autoToolStatus}, Probability: ${autoToolProb}, Type: ${autoToolType}`)
    console.log(`[transferBotdHeaders] Search Bot - Status: ${searchBotStatus}, Probability: ${searchBotProb}, Type: ${searchBotType}`)
    console.log(`[transferBotdHeaders] Virtual Machine - Status: ${vmStatus}, Probability: ${vmProb}, Type: ${vmType}`)
    console.log(`[transferBotdHeaders] Browser Spoofing - Status: ${browserSpoofingStatus}, Probability: ${browserSpoofingProb}, Type: ${browserSpoofingType}`)
  } else
    throw Error(`Unknown status from bot detection server: ${status}`)
}

export async function makeBotDetect(request: Request, config: Config): Promise<Response> {
  try {
    const requestID = getRequestID(request)
    const url = `${config.botdURL}${BOTD_RESULT_PATH}?header&token=${config.token}&id=${requestID}`
    const botRequest = new Request(url.toString(), { method: GET })
    return await fetch(botRequest)
  } catch (e) {
    console.error(`[requestLightDetect] Error handled: ${e.message}`)
    throw Error(`Error during bot detection: ${e.message}`)
  }
}
