import { getPathFromURL, trimURL } from './utils'
import { BOTD_DEFAULT_URL } from './constants'

export default interface Config {
  originURL: string
  botdURL: string
  token: string
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
  botdURL = botdURL == null ? BOTD_DEFAULT_URL : trimURL(botdURL)

  let originURL = await getFromConfig('botd_app')
  originURL = originURL == null ? request.url : trimURL(originURL) + getPathFromURL(request.url)

  console.log(`[getConfig] Config - Botd URL: ${botdURL}, App URL: ${originURL}, Token: ${token}`)

  return { originURL, botdURL, token }
}
