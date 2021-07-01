## example-app

This is simple application which consists login page. Correct credentials:
```bigquery
Login: human
Password: iamnotbot
```

There is a checkbox `Emulate bot` which replaces user-agent to `Chrome Headless`.

### POST `/login`

The handler receives login and password. If integration enabled, request will consist [Botd headers]() as well.
Then, it will check the headers if they exist, and make a decision if it's a bot (any probability more than `0.0`).
In case of positive result, the handler returns [is_bot.html]() page. Otherwise, it checks login and password and returns
[not_bot.html]() page.