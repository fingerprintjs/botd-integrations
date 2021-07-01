import { getConfig } from '../config'
import { makeBotDetect, transferBotdHeaders } from '../detectors/bot'
import { changeURL, isRequestFavicon, isRequestStatic, setErrorHeaders } from '../utils'
import { makeLightDetect, transferLightHeaders } from '../detectors/light'

export default async function handleAll(request: Request): Promise<Response> {
  try {
    console.log(`[handleAll] Request URL: ${request.url}, Method: ${request.method}`)
    const config = await getConfig(request)

    if (isRequestStatic(request)) {
      if (isRequestFavicon(request)) {
        console.log('[handleAll] Request favicon, starting light bot detection')

        const lightDetectResponse = await makeLightDetect(request, config)
        request = changeURL(config.originURL, request)
        transferLightHeaders(lightDetectResponse, request)

        return await fetch(request)
      }
      console.log('[handleAll] Request static data, skipping bot detection')

      const actualRequest = new Request(config.originURL, new Request(request))
      return await fetch(actualRequest)
    }

    const botDetectResponse = await makeBotDetect(request, config)
    request = changeURL(config.originURL, request)
    transferBotdHeaders(botDetectResponse, request)

    return await fetch(request)
  } catch (e) {
    console.error(`[handleAll] Error handled: ${e.message}`)

    setErrorHeaders(request, e)
    return await fetch(request)
  }
}
