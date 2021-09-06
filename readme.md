<img width="960" alt="bot-detection cloud integration-2" src="https://user-images.githubusercontent.com/27387/122214619-f97ab080-ceb2-11eb-8cca-59cdcab33e8b.png">


## FingerprintJS [Botd](https://github.com/fingerprintjs/botd) integrations with cloud platforms.

## Flow with integration enabled

### Example app
Web application that we’re going to protect - http://botd-example-app.fpjs.sh. We will be referring to this app as the **`origin`**.

We'll protect it by adding a CDN layer on top of it, provided by Cloudflare workers, Fastly Compute@Edge, or Amazon Lambda@Edge.

Every CDN example will run middleware functions to intercept requests and responses. These middleware functions are fully open source and are included in this repository.

### Flow with integration enabled
![botd](https://user-images.githubusercontent.com/10922372/126072756-aa246534-2f1c-41d0-b10c-8dc8ea057025.png)

1. End-user loads an example app provided by the integrations ([app powered by Cloudflare](https://botd.fingerprintjs.workers.dev/) or app using [Compute@Edge by Fastly](https://botd-fingerprintjs.edgecompute.app/)).

2. Middleware intercepts first two requests
   (for HTML content of the page and for favicon) and does `light bot detection`
   (sends needed data for light analysis to [Server Botd API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md)).
   On this step we cannot get a lot of useful information to do
   `full bot detection`, we have only information from request (e.g., headers).

3. Middleware sets result of `light bot detection` into headers of request and sends it to origin.

4. Middleware receives response from origin. If it's a request for HTML content it will inject
   [Botd script](https://github.com/fingerprintjs/botd) into the page.

5. Response from origin is returned to end-user's browser with cookie `botd-request-id`. `requestID` value can be used to retrieve the bot detection results later.

6. The end-user fills the form and submits it to the `POST /login` endpoint
   (same logic can be applied for next requests of origin app).

7. Middleware intercepts the request and retrieves results of `full bot detection` from [Server Botd API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md)
   by the botd's `requestID` identifier (available in a `botd-request-id` cookie). Then, it sets the result into headers of the request and
   sends it to origin.

8. Response from origin is returned to end-user's browser.

*Note: If the request retrieves static content (e.g. images, fonts) except favicon, point 7 won't be done.*

Checking the ***Emulate bot*** checkbox will replace `User-Agent` to `Headless Chrome`.
It will force the bot branch of the flow.

### Bot Detection Headers sent to `Origin`

You can find more information about botd headers [here](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md).

#### botd-request-id
Header with request identifier. Example:
`botd-request-id: 6080277c12b178b86f1f967d`.
#### botd-request-status
Possible values of botd-request-status header: `'processed'`, `'inProgress'`, `'error'`.
#### botd-automation-tool-status, botd-browser-spoofing-status, botd-search-bot-status, botd-vm-status
Possible values of status header: `'processed'`, `'error'`, `'notEnoughData'`.
#### botd-automation-tool-prob, botd-browser-spoofing-prob, botd-search-bot-prob, botd-vm-prob
Headers are presented if corresponded `status` is `processed`. The value is float number in range `0.0` to `1.0`.
#### botd-automation-tool-type
**[OPTIONAL]** Possible values: `'phantomjs'`, `'headlessChrome'` and so on.
#### botd-search-bot-type
**[OPTIONAL]** Possible values: `'google'`, `'yandex'` and so on.
#### botd-vm-type
**[OPTIONAL]** Possible values: `'vmware'`, `'parallels'` and so on.
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
