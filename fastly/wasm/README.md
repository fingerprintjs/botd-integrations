# Fastly WASM Botd integration

## Sample integration
![image](https://user-images.githubusercontent.com/10922372/125807555-97e8b4a3-63e7-4a62-9784-e406044702f4.png)
You can try running sample [Fastly Compute@Edge](https://docs.fastly.com/products/compute-at-edge) WASP integration at [https://botd-fingerprintjs.edgecompute.app](https://botd-fingerprintjs.edgecompute.app).

Login: `human`, Password: `iamnotbot`

## Setting up the integration with the pre-built package

### Prerequisites
- Fastly account with the Fastly Compute@Edge [feature flag](https://developer.fastly.com/learning/compute/#create-a-new-fastly-account-and-invite-your-collaborators) enabled.

### Setting up

1. Log in to [manage.fastly.com](https://manage.fastly.com/).

2. Click `Create service`, choose `WASM`, optionally rename service.

3. Go to the `Service configuration`, edit the `Domains` section and add a domain. For testing purposes, you can use `{some-name}.edgecompute.app` format. For more information, take a look at [Fastly documentation](https://developer.fastly.com/learning/concepts/routing-traffic-to-fastly/#computeedge).

4. Edit the `Origins` section.
 
   4.1. Create a new host with the URL of the web application you want to protect, name it `backend`. For demo purposes, you can also use our sample app with `botd-example-app.fpjs.sh` URL. Select correct TLS setting for your app (in most production cases preserve default `Yes, enable TLS and connect securely using port 443`, for our sample app switch to `No, do not enable TLS. Instead connect using port 80`). Update settings.

   4.2. Create a new host with the URL of the botd API - `botd.fpapi.io`, name it `botd`. Stick with the default `Yes, enable TLS and connect securely using port 443` settings.

   4.3. Create a new host with the URL of the CDN - `script-proxy.edgecompute.app`, name it `cdn`. Stick with the default `Yes, enable TLS and connect securely using port 443` settings.

5. Download the `botd-compute-edge-<version>.tar.gz` package from the [releases](https://github.com/fingerprintjs/botd-integrations/releases) and upload it to the `Package` section.

6. Go to the `Dictionaries` section, create a new `botd_config` dictionary.

   6.1. Add item `token` with authorization token obtained from [FingerprintJS](https://fingerprintjs.com/).

   6.2. **[OPTIONAL]** Add item `app_host` with host to origin. You can use our sample host `botd-example-app.fpjs.sh`. We set this value to `HOST` header in each request for bypassing CORS-policy.

   6.3. **[OPTIONAL]** Add item `log_endpoint` with logging endpoint name from `Logging` section.
 
8. Activate integration.

9. Test your app on the provided `Domain` with the given sample credentials.

## Setting up with the source code
If you want to build and release integration from source code, [follow the wiki guidelines](https://github.com/fingerprintjs/botd-integrations/wiki/Setting-up-Fastly-WASM-integration-from-source-code).
