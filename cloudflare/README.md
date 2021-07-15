# Cloudflare Botd integration

## Sample integration
You can try running sample Cloudflare integration at [https://botd.fingerprintjs.workers.dev/](https://botd.fingerprintjs.workers.dev/).

Login: `human`, Password: `iamnotbot`
## Setting up the integration

1. Create an account in [Cloudflare](https://www.cloudflare.com/).

2. Go to the `Workers` section and follow `set up the Wrangler CLI`.

*Note: If `Wrangler login` gets stuck in your console, try [this](https://github.com/cloudflare/wrangler/issues/1703#issuecomment-773797265) method.*

3. Rename `wrangler.toml.example` into `wrangler.toml`.

4. Fill in the parameter `name` - this is the name of the worker to be created.

5. Fill in the parameter `account_id` - this can be found in the Cloudflare dashboard.

6. Create the `CONFIG` namespace. You can learn more about namespaces in the [Cloudflare documentation](https://developers.cloudflare.com/workers/cli-wrangler/commands#kv_namespaces). Replace config binding in `wrangler.toml` with the returned value.

```sh
wrangler kv:namespace create "CONFIG"
```

7. Create a preview namespace for the existing `CONFIG` namespace. Preview namespaces are used for interacting with preview instead of the production environment. This environment is also used for local development. Add returned value to config binding in `wrangler.toml`
```sh
wrangler kv:namespace create "CONFIG" --preview
```

Your `wrangler.toml` should look like:
```toml
name = "<your_integration_name>"
type = "javascript"

account_id = "<your_account_id>"
workers_dev = true
route = ""
zone_id = ""

kv_namespaces = [
    { binding = "CONFIG", preview_id = "<generated_returned_preview_id_by_wrangler_cli>", id = "<generated_returned_preview_id_by_wrangler_cli>" }
]

[build]
command = "npm install && npm run build"
[build.upload]
format = "service-worker"
```

8. Put `botd_token` key-value pair to already created namespace, `botd_token` is an authorization token obtained from [FingerprintJS](https://fingerprintjs.com/).
```sh
wrangler kv:key put --binding=CONFIG "botd_token" "<your_botd_tokent>"
```
*Note: After publishing your worker, you can view and edit your key-value pairs in the `KV` section of the crawler in Cloudflare's UI.*

9. [Optional] Analogically you can set the `botd_token` for your preview environment.
```sh
wrangler kv:key put --binding=CONFIG --preview "botd_token" "<your_botd_tokent>"
```


10. Put `botd_app` key-value pair to already created namespace, `botd_app` is the origin backend URL.
```sh
wrangler kv:key put --binding=CONFIG "botd_app" "<origin_url>"
```

11. [Optional] Analogically you can set the `botd_app` for your preview environment.
```sh
wrangler kv:key put --binding=CONFIG --preview "botd_app" "<origin_url>"
```
 
10. Run `npm install`
  
11. [Optional] If you followed all the optional steps, you can run `wrangler dev` for local testing.

12. Run `wrangler publish` to deploy worker.
