import { Router } from 'itty-router'
import { injectScript } from './injector'
import { makeLightDetect } from './detectors/light'
import { makeBotDetect } from './detectors/bot'
import { getConfig } from './utils'
import {
  AUTO_TOOL_PROB_HEADER, AUTO_TOOL_STATUS_HEADER,
  AUTO_TOOL_TYPE_HEADER, BROWSER_SPOOFING_PROB_HEADER, BROWSER_SPOOFING_STATUS_HEADER, BROWSER_SPOOFING_TYPE_HEADER,
  REQUEST_STATUS_HEADER, SEARCH_BOT_PROB_HEADER, SEARCH_BOT_STATUS_HEADER, SEARCH_BOT_TYPE_HEADER,
  VM_PROB_HEADER, VM_STATUS_HEADER, VM_TYPE_HEADER, REQUEST_ID_HEADER, BOTD_DEFAULT_PATH,
} from './constants'

const router = Router()

router.all('/', handleRequest)
router.all('*', handleAll)

async function gatherResponse(response: Response): Promise<string> {
  const { headers } = response
  const contentType = headers.get('content-type') || ''
  if (contentType.includes('application/json')) {
    return JSON.stringify(await response.json())
  } else if (contentType.includes('application/text')) {
    return await response.text()
  } else if (contentType.includes('text/html')) {
    return await response.text()
  } else {
    return await response.text()
  }
}

async function handleRequest(req: Request) {
  const config = await getConfig()
  const lightDetectResult = await makeLightDetect(req, config)
  console.log(lightDetectResult)

  const originInit = {
    headers: {
      'content-type': 'text/html;charset=UTF-8'
    },
  }
  const response = await fetch(config.backendURL, originInit)
  const results = await gatherResponse(response)
  const injected = injectScript(results, config)

  const clientInit = {
    headers: {
      'content-type': 'text/html;charset=UTF-8',
      SET_COOKIE_HEADER: lightDetectResult.requestID,
      REQUEST_STATUS_HEADER: lightDetectResult.status,

      AUTO_TOOL_STATUS_HEADER: lightDetectResult.result.status,
      AUTO_TOOL_PROB_HEADER: lightDetectResult.result.prob.toFixed(2),
      AUTO_TOOL_TYPE_HEADER: lightDetectResult.result.type
    },
  }

  return new Response(injected, clientInit)
}

async function handleAll(req: Request) {
  const config = await getConfig()
  const oldURL = new URL(req.url)
  const url = config.backendURL + oldURL.pathname

  const botDetectResult = await makeBotDetect(req, config)
  console.log(botDetectResult)
  console.log(botDetectResult.autoTool)

  const newRequest = new Request(url, new Request(req))

  newRequest.headers.append(REQUEST_STATUS_HEADER, botDetectResult.status)
  newRequest.headers.append(REQUEST_ID_HEADER, botDetectResult.requestID)
  newRequest.headers.append(AUTO_TOOL_STATUS_HEADER, botDetectResult.autoTool.status)
  newRequest.headers.append(AUTO_TOOL_PROB_HEADER, botDetectResult.autoTool.prob.toFixed(2))
  newRequest.headers.append(AUTO_TOOL_TYPE_HEADER, botDetectResult.autoTool.type)
  newRequest.headers.append(VM_STATUS_HEADER, botDetectResult.vm.status)
  newRequest.headers.append(VM_PROB_HEADER, botDetectResult.vm.prob.toFixed(2))
  newRequest.headers.append(VM_TYPE_HEADER, botDetectResult.vm.type)
  newRequest.headers.append(BROWSER_SPOOFING_STATUS_HEADER, botDetectResult.browserSpoofing.status)
  newRequest.headers.append(BROWSER_SPOOFING_PROB_HEADER, botDetectResult.browserSpoofing.prob.toFixed(2))
  newRequest.headers.append(BROWSER_SPOOFING_TYPE_HEADER, botDetectResult.browserSpoofing.type)
  newRequest.headers.append(SEARCH_BOT_STATUS_HEADER, botDetectResult.searchBot.status)
  newRequest.headers.append(SEARCH_BOT_PROB_HEADER, botDetectResult.searchBot.prob.toFixed(2))
  newRequest.headers.append(SEARCH_BOT_TYPE_HEADER, botDetectResult.searchBot.type)

  try {
    return await fetch(newRequest)
  } catch (e) {
    return new Response(JSON.stringify({ error: e.message }), { status: 500 })
  }
}

addEventListener('fetch', (e) => {
  e.respondWith(router.handle(e.request))
})
