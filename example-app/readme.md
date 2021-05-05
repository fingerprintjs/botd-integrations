## example-app

Web application that FingerprintJS BotD is going to protect - http://botd-example-app.fpjs.sh

### Application flow

1. End-user loads an example app from http://botd-integration-demo.fpjs.sh/

2. This example app displays a toy version of a login form which FingerprintJS BotD will protect.

3. End-user fills the form and submits it, which makes a POST request to `/login`

4. Example app responds with:

```json
{
  "error": {
    "code": 401,
    "description": "Wrong login or password"
  }
}
```

if login or password are incorrect, or with:
```json
{
  "message": {
    "code": 200,
    "description": "You are successfully logged in!"
  }
}
```
if correct (login: `human`, password: `iamnotbot`).

5. ***Emulate bot*** checkbox is ignored, because there is no bot detection here.
