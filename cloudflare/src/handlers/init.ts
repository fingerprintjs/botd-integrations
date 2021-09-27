import { makeEdgeDetect, transferEdgeHeaders } from '../detectors/edge'
import { injectScript } from '../injector'
import { changeURL, setErrorHeaders } from '../utils'
import { getConfig } from '../config'

export default async function handleInitRequest(request: Request): Promise<Response> {
  try {
    console.log(`[handleInitRequest] Request URL: ${request.url}, Method: ${request.method}`)

    const config = await getConfig(request)
    const edgeDetectResponse = await makeEdgeDetect(request, config)

    request = changeURL(config.originURL, request)
    transferEdgeHeaders(edgeDetectResponse, request)

    const response = await fetch(request)
    console.log(`[handleInitRequest] Origin response - Status: ${response.status}`)

    const html = await response.text()
    const injected = injectScript(html, config)

    return new Response(injected, response)
  } catch (e) {
    console.error(`[handleInitRequest] Error handled: ${e.message}`)

    setErrorHeaders(request, e)
    return await fetch(request)
  }
}
