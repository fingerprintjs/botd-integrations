import { PATH_HASH } from './detect'

export type HeadersDict = Record<string, unknown>

export function cloneReqWithURL(req: Request, newURL: string): Request {
  return new Request(newURL, new Request(req))
}

export function getIP(req: Request): string {
  return req.headers.get("CF-Connecting-IP") || "0.0.0.0"
}

export function getHeadersDict(reqHeaders: Headers): HeadersDict {
  const headersDict: HeadersDict = {}
  for (const [key, value] of reqHeaders) {
    headersDict[key] = [value]
  }
  return headersDict
}

export function trimURL(url: string): string {
  while (url.endsWith('/')) url = url.slice(0, -1)
  return url
}

export function path(url: string): string {
  return new URL(url).pathname
}

export function host(url: string): string {
  return new URL(url).hostname
}

export function query(url: string): string {
  return new URL(url).search
}

export function insert(src: string, i: number, s: string): string {
  return src.substr(0, i) + s + src.substr(i);
}

export function isStatic(req: Request): boolean {
  const STATIC_SEC_FETCH_DEST = ['font', 'script', 'image', 'style', 'video', 'manifest', 'object'] // TODO: add all static types
  const STATIC_PATH_ENDINGS = ['.css', '.js', '.jpg', '.png', '.svg', '.jpeg', '.woff2'] // TODO: add all static types

  // sec-fetch-dest header shows which content was requested, but it works not in all web-browsers
  const secFetchDestOption = req.headers.get('sec-fetch-dest')
  if (secFetchDestOption != null) {
    for (const s of STATIC_SEC_FETCH_DEST) if (s === secFetchDestOption) return true
    return false
  }
  // sec-fetch-dest header doesn't exist => check by path ending
  for (const s of STATIC_PATH_ENDINGS) {
    if (req.url.endsWith(s)) return true
  }
  return false
}

export function isFavicon(req: Request): boolean {
  const p = path(req.url)
  return isStatic(req) && p.endsWith('.ico') && p.indexOf('fav') > -1
}

export function isInit(req: Request): boolean {
  return path(req.url) == "/"
}

export function isDetect(req: Request): boolean {
  return path(req.url) == `/${PATH_HASH}/detect`
}
