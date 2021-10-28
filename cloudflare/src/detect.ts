import * as cookie from 'cookie'
import * as psl from 'psl'
import { getHeadersDict, path, HeadersDict, host } from './utils'
import Config from './config'

export const REQUEST_ID = 'botd-request-id'
export const ERROR_DESCRIPTION_HEADER = 'botd-error-description'
export const REQUEST_STATUS_HEADER = 'botd-request-status'
export const PATH_HASH = "2f70092c";

const enum Status {
  PROCESSED = 'processed',
  ERROR = 'error',
}

interface EdgeDetectBody {
  headers: HeadersDict
  path: string
  previous_request_id: string
  timestamp: number
}

function checkResp(src: Response): boolean {
  const status = src.headers.get(REQUEST_STATUS_HEADER)
  if (status === null) return false

  switch (status) {
    case Status.ERROR: {
      const error = src.headers.get(ERROR_DESCRIPTION_HEADER) || ''
      console.error(`[checkBotdResponse] Handled error from Botd backend: ${error}`)
      return false
    }
    case Status.PROCESSED: { return true }
    default:
      throw Error(`Unknown status from bot detection server: ${status}`)
  }
}

function transferHeaders(src: Response, dst: Request): void {
  const BOTD_HEADERS = [
    REQUEST_ID, REQUEST_STATUS_HEADER, ERROR_DESCRIPTION_HEADER,
    'botd-automation-tool-status', 'botd-automation-tool-prob', 'botd-automation-tool-type',
    'botd-search-bot-status', 'botd-search-bot-prob', 'botd-search-bot-type',
    'botd-browser-spoofing-status', 'botd-browser-spoofing-prob', 'botd-browser-spoofing-type',
    'botd-vm-status', 'botd-vm-prob', 'botd-vm-type'
  ]
  for (const name of BOTD_HEADERS) {
    const value = src.headers.get(name)
    if (value !== null) {
      console.log(`[transferHeaders] Trying set header ${name} with value ${value}`)
      dst.headers.append(name, value)
    }
  }
}

export function makeCookie(req: Request, config: Config): string {
  const domain = psl.get(host(config.realURL)) || undefined
  return cookie.serialize(REQUEST_ID, config.reqId, {
    path: '/',
    secure: true,
    httpOnly: true,
    sameSite: 'none',
    domain: domain,
  })
}

export function getRequestIDFromHeader(r: Request | Response): string {
  return r.headers.get(REQUEST_ID) || ""
}

export function getRequestIDFromCookie(req: Request): string {
  const cookies = cookie.parse(req.headers.get('Cookie') || '');
  return cookies[REQUEST_ID] || ""
}

export function setErrorHeaders(r: Request | Response, e: Error): void {
  r.headers.append(REQUEST_STATUS_HEADER, Status.ERROR)
  r.headers.append(ERROR_DESCRIPTION_HEADER, e.message)
}

export async function botd(req: Request, config: Config): Promise<void> {
  try {
    const botdReq = new Request(`${config.botdURL}/api/v1/results?header&token=${config.token}&id=${config.reqId}`, {
      method: 'Get',
      headers: {
        'botd-client-ip': config.ip
      },
    })
    const resp = await fetch(botdReq)
    checkResp(resp)
    transferHeaders(resp, req)
  } catch (e) {
    console.error(`[botd] Error handled: ${e.message}`)
    throw Error(`Error during bot detection: ${e.message}`)
  }
}

export async function edge(req: Request, config: Config): Promise<void> {
  try {
    const body: EdgeDetectBody = {
      headers: getHeadersDict(req.headers),
      path: path(req.url),
      previous_request_id: config.reqId,
      timestamp: Date.now(),
    }
    const edgeRequest = new Request(`${config.botdURL}/api/v1/edge?header`, {
      method: 'POST',
      body: JSON.stringify(body),
      headers: {
        'auth-token': config.token,
        'botd-client-ip': config.ip
      },
    })
    const resp = await fetch(edgeRequest)
    checkResp(resp)
    transferHeaders(resp, req)
    config.reqId = getRequestIDFromHeader(resp)
  } catch (e) {
    console.error(`[edge] Error handled: ${e.message}`)
    throw Error(`Error during edge bot detection: ${e.message}`)
  }
}
