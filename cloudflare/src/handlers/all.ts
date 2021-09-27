import { getConfig } from '../config'
import { makeBotDetect, transferBotdHeaders } from '../detectors/bot'
import { changeURL, isRequestFavicon, isRequestStatic, setErrorHeaders } from '../utils'
import { makeEdgeDetect, transferEdgeHeaders } from '../detectors/edge'

export default async function handleAll(request: Request): Promise<Response> {
  try {
    console.log(`[handleAll] Request URL: ${request.url}, Method: ${request.method}`)
    const config = await getConfig(request)

    if (isRequestStatic(request)) {
      if (isRequestFavicon(request)) {
        console.log('[handleAll] Request favicon, starting edge bot detection')

        const edgeDetectResponse = await makeEdgeDetect(request, config)
        request = changeURL(config.originURL, request)
        transferEdgeHeaders(edgeDetectResponse, request)

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
