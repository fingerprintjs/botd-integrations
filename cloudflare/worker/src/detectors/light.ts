import { getHeadersDict, getPathFromURL, getRequestID, HeadersDict } from '../utils'
import Config from '../config'
import { AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER, AUTO_TOOL_TYPE_HEADER,
  BOTD_LIGHT_PATH, ERROR_DESCRIPTION_HEADER, POST,
  REQUEST_ID_HEADER, REQUEST_STATUS_HEADER, SET_COOKIE_HEADER, Status,
} from '../constants'

interface LightDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

export function transferLightHeaders(src: Response, dst: Request): void {
  const status = src.headers.get(REQUEST_STATUS_HEADER) || ''
  dst.headers.append(REQUEST_STATUS_HEADER, status)

  if (status === Status.ERROR) {
    const error = src.headers.get(ERROR_DESCRIPTION_HEADER) || ''
    dst.headers.append(ERROR_DESCRIPTION_HEADER, error)

    console.error(`[transferLightHeaders] Handled error from Botd backend: ${error}`)

  } else if (status === Status.PROCESSED) {
    const requestID = src.headers.get(REQUEST_ID_HEADER) || ''
    const autoToolStatus =  src.headers.get(AUTO_TOOL_STATUS_HEADER) || ''
    const autoToolProb = src.headers.get(AUTO_TOOL_PROB_HEADER) || ''
    const autoToolType = src.headers.get(AUTO_TOOL_TYPE_HEADER) || ''

    dst.headers.append(SET_COOKIE_HEADER, requestID)
    dst.headers.append(AUTO_TOOL_STATUS_HEADER, autoToolStatus)
    dst.headers.append(AUTO_TOOL_PROB_HEADER, autoToolProb)
    dst.headers.append(AUTO_TOOL_TYPE_HEADER, autoToolType)

    console.log(`[transferLightHeaders] Light Detect Result - Status: ${status}, Request ID: ${requestID}`)
    console.log(`[transferLightHeaders] Automation Tool - Status: ${autoToolStatus}, Probability: ${autoToolProb}, Type: ${autoToolType}`)
  } else
    throw Error(`Unknown status from bot detection server: ${status}`)
}

export async function makeLightDetect(request: Request, config: Config): Promise<Response> {
  try {
    const body: LightDetectBody = {
      headers: getHeadersDict(request.headers),
      path: getPathFromURL(request.url),
      previous_request_id: getRequestID(request),
      timestamp: Date.now()
    }
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
    throw Error(`Error during light bot detection: ${ e.message }`)
  }
}
