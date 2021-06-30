import {
  COOKIE_HEADER,
  COOKIE_NAME,
  ERROR_DESCRIPTION_HEADER,
  REQUEST_STATUS_HEADER,
  SEC_FETCH_DEST_HEADER,
  STATIC_PATH_ENDINGS,
  STATIC_SEC_FETCH_DEST, Status,
} from './constants'
import {HeadersDict} from "./types";

export function changeURL(newURL: string, request: Request): Request {
  return new Request(newURL, new Request(request))
}

function getCookie(cookie: string, name: string): string | undefined {
  const matches = cookie.match(
    new RegExp('(?:^|; )' + name.replace(/([.$?*|{}()[\]\\/+^])/g, '\\$1') + '=([^;]*)'),
  )
  return matches ? decodeURIComponent(matches[1]) : undefined
}

export function getRequestID(request: Request): string {
  const cookies = request.headers.get(COOKIE_HEADER) || ""
  return getCookie(cookies, COOKIE_NAME) || ""
}

export function getHeadersDict(requestHeaders: Headers): HeadersDict {
  const headersDict: HeadersDict = {}
  for (const [key, value] of requestHeaders) {
    headersDict[key] = [value]
  }
  return headersDict
}

export function trimURL(url: string ): string {
  while (url.endsWith('/'))
    url = url.slice(0, -1)
  return url
}

export function setErrorHeaders(r: Request | Response, e: Error): void {
  r.headers.append(REQUEST_STATUS_HEADER, Status.ERROR)
  r.headers.append(ERROR_DESCRIPTION_HEADER, e.message)
}

export function getPathFromURL(url: string): string {
  return (new URL(url)).pathname
}

export function isRequestStatic(request: Request): boolean {
  // sec-fetch-dest header shows which content was requested, but it works not in all web-browsers
  const secFetchDestOption = request.headers.get(SEC_FETCH_DEST_HEADER)
  if (secFetchDestOption != null) {
    for (const s of STATIC_SEC_FETCH_DEST)
      if (s === secFetchDestOption)
        return true
    return false;
  }
  // sec-fetch-dest header doesn't exist => check by path ending
  for (const s in STATIC_PATH_ENDINGS){
    if (request.url.endsWith(s))
      return true;
  }
  return false;
}

export function isRequestFavicon(request: Request): boolean {
  const path = getPathFromURL(request.url)
  return (path.endsWith(".ico") && path.indexOf("fav") > -1)
}
