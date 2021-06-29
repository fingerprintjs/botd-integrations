import { makeLightDetect, setLightDetectHeaders } from '../detectors/light'
import { injectScript } from '../injector'
import { getPathFromURL, setErrorHeaders } from '../utils'
import { getConfig } from '../config'

export default async function handleInitRequest(request: Request): Promise<Response> {
  try {
    console.log(`[handleInitRequest] Request URL: ${request.url}, Method: ${request.method}`)

    const config = await getConfig(request)
    const lightDetectResult = await makeLightDetect(request, config)

    const actualURL = config.backendURL + getPathFromURL(request.url)
    const actualRequest = new Request(actualURL, new Request(request))
    setLightDetectHeaders(actualRequest, lightDetectResult)

    const response = await fetch(actualRequest)
    const html = await response.text()
    const injected = injectScript(html, config)

    return new Response(injected, response)

  } catch (e) {
    console.error(`[handleInitRequest] Error handled: ${ e.message }`)

    setErrorHeaders(request, e)
    return await fetch(request)
  }
}
