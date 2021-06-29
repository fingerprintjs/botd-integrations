import { getConfig } from '../config'
import { makeBotDetect, setBotDetectHeaders } from '../detectors/bot'
import { getPathFromURL, isRequestFavicon, isRequestStatic, setErrorHeaders } from '../utils'
import { makeLightDetect, setLightDetectHeaders } from '../detectors/light'

export default async function handleAll(request: Request): Promise<Response> {
  try {
    console.log(`[handleAll] Request URL: ${request.url}, Method: ${request.method}`)
    const config = await getConfig(request)

    if (isRequestStatic(request)) {
      if (isRequestFavicon(request)) {
        console.log("[handleAll] Request favicon, starting light bot detection")

        const lightDetectResult = await makeLightDetect(request, config)
        const actualURL = config.backendURL + getPathFromURL(request.url)
        const actualRequest = new Request(actualURL, new Request(request))
        setLightDetectHeaders(actualRequest, lightDetectResult)

        return await fetch(actualRequest)
      }
      console.log("[handleAll] Request static data, skipping bot detection")

      const actualURL = config.backendURL + getPathFromURL(request.url)
      const actualRequest = new Request(actualURL, new Request(request))

      return await fetch(actualRequest)
    }

    const botDetectResult = await makeBotDetect(request, config)

    const actualURL = config.backendURL + getPathFromURL(request.url)
    const actualRequest = new Request(actualURL, new Request(request))
    setBotDetectHeaders(actualRequest, botDetectResult)

    return await fetch(actualRequest)
  } catch (e) {
    console.error(`[handleAll] Error handled: ${ e.message }`)

    setErrorHeaders(request, e)
    return await fetch(request)
  }
}
