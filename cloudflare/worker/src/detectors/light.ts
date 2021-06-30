import {getHeadersDict, getPathFromURL, getRequestID, HeadersDict} from '../utils'
import Config from '../config'
import {
  BOTD_LIGHT_PATH, ERROR_DESCRIPTION_HEADER, LIGHT_RESULT_HEADERS, POST,
  REQUEST_ID_HEADER, REQUEST_STATUS_HEADER, RESULT_HEADERS, SET_COOKIE_HEADER, Status,
} from '../constants'

interface LightDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

export function transferLightHeaders(src: Response, dst: Request): void {
  const s = src.headers
  const d = dst.headers

  const status = s.get(REQUEST_STATUS_HEADER) || ''

  if (status === Status.ERROR) {
    d.append(REQUEST_STATUS_HEADER, status)
    const error = s.get(ERROR_DESCRIPTION_HEADER) || ''
    d.append(ERROR_DESCRIPTION_HEADER, error)
    console.error(`[transferLightHeaders] Handled error from Botd backend: ${error}`)

  } else if (status === Status.PROCESSED) {
    for (const name in LIGHT_RESULT_HEADERS) {
      const value = s.get(name) || ''
      d.append(name, value)
      console.log(`[transferLightHeaders] Header: ${name}, Value: ${value}`)
    }

    const requestId = s.get(REQUEST_ID_HEADER) || ''
    d.append(SET_COOKIE_HEADER, requestId)

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
