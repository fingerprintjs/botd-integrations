import { getRequestID } from '../utils'
import Config from '../config'
import {
  BOTD_RESULT_PATH,
  REQUEST_STATUS_HEADER,
  GET,
  ERROR_DESCRIPTION_HEADER,
  Status,
  RESULT_HEADERS,
} from '../constants'

export function transferBotdHeaders(src: Response, dst: Request): void {
  const s = src.headers
  const d = dst.headers

  const status = s.get(REQUEST_STATUS_HEADER) || ''

  switch (status) {
    case Status.ERROR: {
      const error = s.get(ERROR_DESCRIPTION_HEADER) || ''
      d.append(REQUEST_STATUS_HEADER, status)
      d.append(ERROR_DESCRIPTION_HEADER, error)
      console.error(`[transferBotdHeaders] Handled error from Botd backend: ${error}`)
      break
    }

    case Status.PROCESSED: {
      for (const name of RESULT_HEADERS) {
        const value = s.get(name) || ''
        d.append(name, value)
        console.log(`[transferBotdHeaders] Header: ${name}, Value: ${value}`)
      }
      break
    }

    default:
      throw Error(`Unknown status from bot detection server: ${status}`)
  }
}

export async function makeBotDetect(request: Request, config: Config): Promise<Response> {
  try {
    const requestID = getRequestID(request)
    const url = `${config.botdURL}${BOTD_RESULT_PATH}?header&token=${config.token}&id=${requestID}`
    const botRequest = new Request(url.toString(), { method: GET })
    return await fetch(botRequest)
  } catch (e) {
    console.error(`[requestEdgeDetect] Error handled: ${e.message}`)
    throw Error(`Error during bot detection: ${e.message}`)
  }
}
