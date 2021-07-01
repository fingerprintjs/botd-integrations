# botd integrations
<img width="1280" alt="bot-detection cloud integration-2" src="https://user-images.githubusercontent.com/27387/122214619-f97ab080-ceb2-11eb-8cca-59cdcab33e8b.png">


## FingerprintJS Botd integrations with cloud providers.

## Flow with integration enabled

### Example app
Web application that we’re going to protect with Fastly - http://botd-example-app.fpjs.sh (origin).

Middleware examples: Cloudflare worker, Fastly Compute@Edge function, Amazon Lambda@Edge.

### Flow with integration enabled
1. End-user loads an example app.

2. Middleware intercepts first two requests
   (for HTML content of the page and for favicon) and does `light bot detection`
   (sends needed data for light analysis to [Server Botd API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md)).
   On this step we cannot get a lot of useful information to do
   `full bot detection`, we have only information from request (e.g., headers).

3. Middleware sets result of `light bot detection` into headers of request and sends it to origin.

4. Middleware receives response from origin. If it's a request for HTML content it will inject
   [Botd script](https://github.com/fingerprintjs/botd) into the page.

5. Response from origin returns to client's browser with cookie `botd-request-id`.

6. The end-user fills the form and submits it to the `POST /login` endpoint
   (same logic can be applied for next requests of origin app).

7. Middleware intercepts the request and retrieves results of `full bot detection` from [Server Botd API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md)
   by the botd request identifier (available in a cookie). Then, it sets the result into headers of the request and
   sends it to origin.

8. Response from origin returns to client's browser.

9. If the request retrieves static content (e.g. images, fonts) except favicon, point 7 won't be done.

Checking the ***Emulate bot*** checkbox will replace `User-Agent` to `Headless Chrome`.
It will force the bot branch of the flow.

### Origin Bot Detection Headers

More details about data in the headers you can find [here](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md).

#### botd-request-id
Header with request id. Example:
`botd-request-id: 6080277c12b178b86f1f967d`
#### botd-request-status
Possible values of botd-request-status header = ["processed" | "inProgress" | "error"]
#### botd-automation-tool-status, botd-browser-spoofing-status, botd-search-bot-status, botd-vm-status
Possible values of status header = ["processed" | "error" | "notEnoughData"]
#### botd-automation-tool-prob, botd-browser-spoofing-prob, botd-search-bot-prob, botd-vm-prob
Headers are presented if `status` is `processed`. Possible values = [0.0 .. 1.0]
#### botd-automation-tool-type
**[OPTIONAL]** Possible values = ["phantomjs", "headlessChrome", ...]
#### botd-search-bot-type
**[OPTIONAL]** Possible values = ["google", "yandex" ...]
#### botd-vm-type
**[OPTIONAL]** Possible values = ["vmware", "parallels" ...]
### Headers example:
```
botd-request-id: 6080277c12b178b86f1f967d
botd-request-status: processed

botd-automation-tool-status: processed
botd-automation-tool-prob: 0.00

botd-browser-spoofing-status: processed
botd-browser-spoofing-prob: 0.00

botd-search-bot-status: processed
botd-search-bot-prob: 0.00

botd-vm-status: processed
botd-vm-prob: 0.00
```
### Headers example, when an error occurred:
```
botd-request-id: 6080277c12b178b86f1f967
botd-request-status: error
botd-error-description: token not found
```
