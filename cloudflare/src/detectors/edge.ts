import { getHeadersDict, getPathFromURL, getRequestID, HeadersDict } from '../utils'
import Config from '../config'
import {
  BOTD_EDGE_PATH,
  COOKIE_NAME,
  ERROR_DESCRIPTION_HEADER,
  EDGE_RESULT_HEADERS,
  POST,
  REQUEST_ID_HEADER,
  REQUEST_STATUS_HEADER,
  SET_COOKIE_HEADER,
  Status,
} from '../constants'

interface EdgeDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

export function transferEdgeHeaders(src: Response, dst: Request): void {
  const s = src.headers
  const d = dst.headers

  const status = s.get(REQUEST_STATUS_HEADER) || ''

  switch (status) {
    case Status.ERROR: {
      d.append(REQUEST_STATUS_HEADER, status)
      const error = s.get(ERROR_DESCRIPTION_HEADER) || ''
      d.append(ERROR_DESCRIPTION_HEADER, error)
      console.error(`[transferEdgeHeaders] Handled error from Botd backend: ${error}`)
      break
    }

    case Status.PROCESSED: {
      for (const name of EDGE_RESULT_HEADERS) {
        const value = s.get(name) || ''
        d.append(name, value)
        console.log(`[transferEdgeHeaders] Header: ${name}, Value: ${value}`)
      }

      const requestId = s.get(REQUEST_ID_HEADER) || ''
      const cookie = `${COOKIE_NAME}=${requestId}`
      d.append(SET_COOKIE_HEADER, cookie)
      break
    }

    default:
      throw Error(`Unknown status from bot detection server: ${status}`)
  }
}

export async function makeEdgeDetect(request: Request, config: Config): Promise<Response> {
  try {
    const body: EdgeDetectBody = {
      headers: getHeadersDict(request.headers),
      path: getPathFromURL(request.url),
      previous_request_id: getRequestID(request),
      timestamp: Date.now(),
    }
    const edgeRequestInit = {
      method: POST,
      body: JSON.stringify(body),
      headers: { 'Auth-Token': config.token },
    }
    const url = `${config.botdURL}${BOTD_EDGE_PATH}?header`
    const edgeRequest = new Request(url, edgeRequestInit)
    return await fetch(edgeRequest)
  } catch (e) {
    console.error(`[requestEdgeDetect] Error handled: ${e.message}`)
    throw Error(`Error during edge bot detection: ${e.message}`)
  }
}
