import { getIP, path, trimURL } from './utils'
import { getRequestIDFromCookie } from './detect'

export default interface Config {
  originURL: string
  botdURL: string
  realURL: string
  token: string
  ip: string
  reqId: string
}

async function getFromConfig(key: string): Promise<string | null> {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return await CONFIG.get(key)
}

export async function getConfig(request: Request): Promise<Config> {
  const token = await getFromConfig('botd_token')
  if (token == null) throw Error('Can`t find botd token in KV storage')

  let botdURL = await getFromConfig('botd_url')
  botdURL = botdURL == null ? 'https://botd.fpapi.io' : trimURL(botdURL)

  let originURL = await getFromConfig('botd_app')
  originURL = originURL == null ? request.url : trimURL(originURL) + path(request.url)

  const ip = getIP(request)
  const reqId = getRequestIDFromCookie(request)
  const realURL = request.url
  console.log(`[getConfig] Config - Botd URL: ${botdURL}, App URL: ${originURL}, Token: ${token}, IP: ${ip}, request ID: ${reqId}`)

  return { originURL, botdURL, realURL, token, ip, reqId }
}
