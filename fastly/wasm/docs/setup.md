## How to set up
1. Go to [manage.fastly.com](https://manage.fastly.com/)

2. Click `Create service`, then choose `WASM`
   
3. Install the [Fastly CLI](https://github.com/fastly/cli)

4. Run `fastly compute init` and follow the prompts to create a project

5. Download the source code from this repository and replace `src` folder in your local project

6. Edit `Domains`

7. Edit `Health checks`

8. Edit `Hosts`

   8.1. Create origin host, name should be `Backend`

   8.2. Create Bot Detection API host, name should be `Botd`, url is `botd.fpapi.io`

9. Edit `Dictionaries`

   9.1. Create `config` dictionary

   9.2. Add item `app_backend_url` with url to origin (e.x. `http://botd-example-app.fpjs.sh`)

   9.3. Add item `botd_token` with authorization token

   9.4. **[OPTIONAL]** Add item `botd_url` with url to Bot Detection API (default `https://botd.fpapi.io/`)

   9.5. **[OPTIONAL]** Add item `env` - environment name for logging (default `Middleware`)

10. Edit `Logging`

11. Run `fastly compute publish --service-id={serviceId}`

### Development

Building an application: `fastly compute build`

Deploying an application: `fastly compute deploy -s {serviceId}`
