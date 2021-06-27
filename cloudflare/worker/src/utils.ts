import { COOKIE_HEADER, COOKIE_NAME } from './constants'

export type HeadersDict = Record<string, unknown>

export interface DetectResultItem {
  status: string
  prob: number
  type: string
}

export interface Config {
  backendURL: string
  botdURL: string
  token: string
}

function getCookie(cookie: string, name: string): string | undefined {
  const matches = cookie.match(
    new RegExp('(?:^|; )' + name.replace(/([.$?*|{}()[\]\\/+^])/g, '\\$1') + '=([^;]*)'),
  )
  return matches ? decodeURIComponent(matches[1]) : undefined
}

export function getRequestID(request: Request): string {
  const cookies = request.headers.get(COOKIE_HEADER) || ""
  return getCookie(cookies, COOKIE_NAME) || ""
}

async function getFromConfig(key: string): Promise<string> {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const value = await CONFIG.get(key)
  console.log(`Get from config: ${key}: ${value}`)
  return value
}

export async function getConfig(): Promise<Config> {
  return {
    backendURL: await getFromConfig("app_backend_url"),
    botdURL: await getFromConfig("botd_url"),
    token: await getFromConfig("token")
  }
}
