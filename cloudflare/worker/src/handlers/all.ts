import { getConfig } from '../config'
import { makeBotDetect, setBotDetectHeaders } from '../detectors/bot'
import { changeURL, isRequestFavicon, isRequestStatic, setErrorHeaders } from '../utils'
import { makeLightDetect, setLightDetectHeaders } from '../detectors/light'

export default async function handleAll(request: Request): Promise<Response> {
  try {
    console.log(`[handleAll] Request URL: ${request.url}, Method: ${request.method}`)
    const config = await getConfig(request)

    if (isRequestStatic(request)) {
      if (isRequestFavicon(request)) {
        console.log("[handleAll] Request favicon, starting light bot detection")

        const lightDetectResult = await makeLightDetect(request, config)
        request = changeURL(config.backendURL, request)
        setLightDetectHeaders(request, lightDetectResult)

        return await fetch(request)
      }
      console.log("[handleAll] Request static data, skipping bot detection")

      const actualRequest = new Request(config.backendURL, new Request(request))
      return await fetch(actualRequest)
    }

    const botDetectResult = await makeBotDetect(request, config)
    request = changeURL(config.backendURL, request)
    setBotDetectHeaders(request, botDetectResult)

    return await fetch(request)
  } catch (e) {
    console.error(`[handleAll] Error handled: ${ e.message }`)

    setErrorHeaders(request, e)
    return await fetch(request)
  }
}
