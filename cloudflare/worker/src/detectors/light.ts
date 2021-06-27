import { Config, DetectResultItem, getRequestID, HeadersDict } from '../utils'
import {
  BOTD_LIGHT_PATH, REQUEST_ID_HEADER, REQUEST_STATUS_HEADER,
  LIGHT_PROB_HEADER, LIGHT_STATUS_HEADER, LIGHT_TYPE_HEADER, AUTH_TOKEN_HEADER
} from '../constants'

export interface LightDetectResult {
  requestID: string
  status: string
  result: DetectResultItem
}

export interface LightDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

export async function makeLightDetect(request: Request, config: Config): Promise<LightDetectResult> {
  const requestID = getRequestID(request)
  const timestamp = Date.now()

  const headers: HeadersDict = {}
  for (const [key, value] of request.headers) {
    headers[key] = [value]
  }

  const body: LightDetectBody = {
    headers: headers,
    path: new URL(request.url).pathname,
    previous_request_id: requestID,
    timestamp: timestamp
  }

  const lightResponse = await requestLightDetect(body, config)
  return lightResultFromHeaders(lightResponse.headers)
}

function lightResultFromHeaders(headers: Headers): LightDetectResult {
  const request_status = headers.get(REQUEST_STATUS_HEADER) || ""
  const request_id = headers.get(REQUEST_ID_HEADER) || ""
  const light_status = headers.get(LIGHT_STATUS_HEADER) || ""
  const light_probability = headers.get(LIGHT_PROB_HEADER) || ""
  const light_type = headers.get(LIGHT_TYPE_HEADER) || ""

  return {
    requestID: request_id,
    status: request_status,
    result: {
      status: light_status,
      prob: parseFloat(light_probability),
      type: light_type
    }
  }
}

async function requestLightDetect(body: LightDetectBody, config: Config): Promise<Response> {
  const lightRequestInit = {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      AUTH_TOKEN_HEADER: config.token
    },
  }
  const url = new URL(config.botdURL + BOTD_LIGHT_PATH)
  console.log(url.toString())
  const lightRequest = new Request(url.toString(), lightRequestInit)

  try {
    return await fetch(lightRequest);
  } catch (e) {
    console.error(e.message)
    return new Response(JSON.stringify({ error: e.message }), { status: 500 });
  }
}
