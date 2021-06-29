import { DetectResultError, DetectResultItem, HeadersDict, getHeadersDict, getPathFromURL, getRequestID } from '../utils'
import Config from '../config'
import {
  BOTD_LIGHT_PATH, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER,
  AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_PROB_HEADER, AUTO_TOOL_TYPE_HEADER,
  SET_COOKIE_HEADER, POST, ERROR_DESCRIPTION_HEADER, Status,
} from '../constants'

type LightDetectResult = LightDetectResultProcessed | DetectResultError

interface LightDetectResultProcessed {
  requestID: string
  status: Status.PROCESSED
  result: DetectResultItem
}

interface LightDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

function lightResultFromHeaders(headers: Headers): LightDetectResult {
  const status = headers.get(REQUEST_STATUS_HEADER) || ""

  if (status === Status.ERROR) {
    const error = headers.get(ERROR_DESCRIPTION_HEADER) || ''
    console.error(`[lightResultFromHeaders] Handled error from Botd backend: ${error}`)
    return { status, error }

  } else if (status === Status.PROCESSED) {
    const request_id = headers.get(REQUEST_ID_HEADER) || ""
    const light_status = headers.get(AUTO_TOOL_STATUS_HEADER) || ""
    const light_probability = headers.get(AUTO_TOOL_PROB_HEADER) || ""
    const light_type = headers.get(AUTO_TOOL_TYPE_HEADER) || ""

    return {
      requestID: request_id,
      status: status,
      result: {
        status: light_status,
        prob: parseFloat(light_probability),
        type: light_type
      }
    }
  } else
    throw Error(`Unknown status from bot detection server: ${status}`)
}

async function requestLightDetect(body: LightDetectBody, config: Config): Promise<Response> {
  try {
    const lightRequestInit = {
      method: POST,
      body: JSON.stringify(body),
      headers: { 'Auth-Token': config.token },
    }
    const url = config.botdURL + BOTD_LIGHT_PATH
    const lightRequest = new Request(url, lightRequestInit)
    return await fetch(lightRequest);
  } catch (e) {
    console.error(`[requestLightDetect] Error handled: ${ e.message }`)
    throw Error(`Error during light bot detection: ${e.message}`)
  }
}

export function setLightDetectHeaders(r: Response | Request, results: LightDetectResult): void {
  r.headers.append(REQUEST_STATUS_HEADER, results.status)

  if (results.status === Status.ERROR)
    r.headers.append(ERROR_DESCRIPTION_HEADER, results.error)

  else {
    r.headers.append(SET_COOKIE_HEADER, results.requestID)
    r.headers.append(AUTO_TOOL_STATUS_HEADER, results.result.status)
    r.headers.append(AUTO_TOOL_PROB_HEADER, results.result.prob.toFixed(2))
    r.headers.append(AUTO_TOOL_TYPE_HEADER, results.result.type)
  }
}

export async function makeLightDetect(request: Request, config: Config): Promise<LightDetectResult> {
  const body: LightDetectBody = {
    headers: getHeadersDict(request.headers),
    path: getPathFromURL(request.url),
    previous_request_id: getRequestID(request),
    timestamp: Date.now()
  }

  const lightResponse = await requestLightDetect(body, config)
  const lightResult = lightResultFromHeaders(lightResponse.headers)

  if (lightResult.status === Status.ERROR) {
    console.log(`[makeBotDetect] Bot Detect Result - Status: ${lightResult.status}, Error message: ${lightResult.error}`)
  } else {
    console.log(`[makeLightDetect] Light Detect Result - Status: ${lightResult.status}, Request ID: ${lightResult.requestID}`)
    console.log(`[makeLightDetect] Automation Tool - Status: ${lightResult.result.status}, Probability: ${lightResult.result.prob}, Type: ${lightResult.result.type}`)
  }

  return lightResult
}
