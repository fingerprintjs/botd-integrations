## Example app
Web application that we’re going to protect with Fastly - http://botd-integration-demo.fpjs.sh (origin).

## Flow with Fastly integration enabled
1. End-user loads an example app from https://fpjs-wasm.edgecompute.app/.

2. Fastly Compute@Edge function (С@E function) intercepts first two requests 
   (for HTML content of the page and for favicon) and does `light bot detection` 
   (sends needed data for light analysis to [Bot Detection API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md)).
   On this step we cannot get a lot of useful information to do 
   `full bot detection`, we have only request information (e.g. headers).
   
3. С@E function sets result of `light bot detection` into headers of request and sends it to origin.

4. С@E function receives response from origin. If it's a request for HTML content it will inject 
   [botd script](https://github.com/fingerprintjs/botd) into the page.

5. Response from origin returns to client's browser with cookie `botd-request-id`.
  
6. The end-user fills the form and submits it to the `POST /login` endpoint 
   (same logic can be applied for next requests of origin app).

7. С@E function intercepts the request and retrieves results of `full bot detection` from [Bot Detection API](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md) 
   by the botd request identifier (available in a cookie). Then, it sets the result into headers of the request and 
   sends it to origin.

8. Response from origin returns to client's browser.

9. TODO: If the request retrieves static content (e.g. images, fonts) except favicon, point 7 won't be done. 

Checking the ***Emulate bot*** checkbox will replace `User-Agent` to `Headless Chrome`. 
It will force the bot branch of the flow.

Below is the flow diagram:

![](resources/diagram.jpg)

## Origin Bot Detection Headers

More details about data in the headers you can find [here](https://github.com/fingerprintjs/botd/blob/main/docs/server_api.md).

#### fpjs-request-id
Header with request id. Example:
`fpjs-request-id: 6080277c12b178b86f1f967d`
#### fpjs-request-status
Possible values of fpjs-request-status header = ["processed" | "inProgress" | "error"]
#### fpjs-bot-status, fpjs-browser-spoofing-status, fpjs-search-bot-status, fpjs-vm-status
Possible values of status header = ["processed" | "error" | "notEnoughData"]
#### fpjs-bot-prob, fpjs-browser-spoofing-prob, fpjs-search-bot-prob, fpjs-vm-prob
Headers are presented if `status` is `processed`. Possible values = [0.0 .. 1.0]
#### fpjs-bot-type
**[OPTIONAL]** Possible values = ["phantomjs", "headlessChrome", ...]
#### fpjs-search-bot-type
**[OPTIONAL]** Possible values = ["google", "yandex" ...]
#### fpjs-vm-type
**[OPTIONAL]** Possible values = ["vmware", "parallels" ...]
### Headers example:
```
fpjs-request-id: 6080277c12b178b86f1f967d
fpjs-request-status: processed

fpjs-bot-status: processed
fpjs-bot-prob: 0.00

fpjs-browser-spoofing-status: processed
fpjs-browser-spoofing-prob: 0.00

fpjs-search-bot-status: processed
fpjs-search-bot-prob: 0.00

fpjs-vm-status: processed
fpjs-vm-prob: 0.00
```
### Headers example, when an error occurred:
```
fpjs-request-id: 6080277c12b178b86f1f967
fpjs-request-status: error
fpjs-error-description: token not found
```
