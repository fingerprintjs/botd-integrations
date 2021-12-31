import Config, { getConfig } from './config'
import { cloneReqWithURL, isFavicon, isStatic, isInit, isDetect, query } from './utils'
import { botd, edge, makeCookie, setErrorHeaders } from './detect'
import { injectScript } from './injector'

async function initHandler(req: Request, config: Config): Promise<Response> {
  console.log(`[init] Initial request => starting edge bot detection`)
  await edge(req, config)

  let resp = await fetch(req)
  console.log(`[init] Origin response finished with status: ${resp.status}, starting code injection...`)
  resp = injectScript(resp, config)

  const cookie = makeCookie(req, config)
  console.log(`[init] Setting cookie ${cookie}`)
  resp.headers.append('set-cookie', cookie)
  return resp
}

async function detectHandler(req: Request, config: Config): Promise<Response> {
  console.log('[detect] Detect request => redirecting to Botd')
  const queryString = query(config.realURL)
  const url = `${config.botdURL}/api/v1/detect${queryString}`
  const botdReq = cloneReqWithURL(req, url)
  botdReq.headers.append('botd-client-ip', config.ip)
  const resp = await fetch(botdReq)
  const body = await resp.text()
  const bodyJSON = JSON.parse(body)
  config.reqId = bodyJSON["requestId"]
  const cookie = makeCookie(req, config)
  console.log(`[init] Setting cookie ${cookie}`)

  const mutResp = new Response(body, resp)
  mutResp.headers.append('set-cookie', cookie)

  return mutResp
}

async function faviconHandler(req: Request, config: Config): Promise<Response> {
  console.log('[favicon] Request favicon => starting edge bot detection')
  await edge(req, config)
  return await fetch(req)
}

async function staticHandler(req: Request): Promise<Response> {
  console.log('[static] Request static data => skipping bot detection')
  return await fetch(req)
}

async function nonStaticHandler(req: Request, config: Config): Promise<Response> {
  await botd(req, config)
  const resp = await fetch(req)
  const mutResp = new Response(resp.body, resp)
  mutResp.headers.append('set-cookie', makeCookie(req, config))
  return mutResp
}

async function errorHandler(req: Request, e: Error): Promise<Response> {
  console.error(`[error] Error handled: ${e.message}`)
  setErrorHeaders(req, e)
  return await fetch(req)
}

async function allHandler(r: Request): Promise<Response> {
  try {
    console.log(`[index] Request URL: ${r.url}, Method: ${r.method}`)
    const config = await getConfig(r)
    const req = cloneReqWithURL(r, config.originURL)

    if (isInit(req))
      return initHandler(req, config)
    else if (isDetect(req))
      return detectHandler(req, config)
    else if (isFavicon(req))
      return faviconHandler(req, config)
    else if (isStatic(req))
      return staticHandler(req)
    else
      return nonStaticHandler(req, config)
  } catch (e) {
    return errorHandler(r, e)
  }
}

addEventListener('fetch', (e) => { e.respondWith(allHandler(e.request)) })
