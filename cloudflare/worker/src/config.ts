import { getPathFromURL, removeLastSlash } from './utils'
import { BOTD_DEFAULT_URL } from './constants'

export default interface Config {
  backendURL: string
  botdURL: string
  token: string
}

async function getFromConfig(key: string): Promise<string | null> {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return await CONFIG.get(key)
}

export async function getConfig(request: Request): Promise<Config> {
  const token = await getFromConfig("botd_token")
  if (token == null)
    throw Error("Can`t find botd token in KV storage")

  let botdURL = await getFromConfig("botd_url")
  if (botdURL == null)
    botdURL = BOTD_DEFAULT_URL
  else
    botdURL = removeLastSlash(botdURL)

  let backendURL = await getFromConfig("botd_app")
  if (backendURL != null)
    backendURL = removeLastSlash(backendURL) + getPathFromURL(request.url)
  else
    backendURL = request.url

  console.log(`[getConfig] Config - Botd URL: ${botdURL}, App URL: ${backendURL}, Token: ${token}`)

  return { backendURL, botdURL, token }
}
