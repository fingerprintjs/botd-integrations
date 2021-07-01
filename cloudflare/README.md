## Cloudflare Botd integration

### Setting up the integration

1. Create an account in Cloudflare

2. Go to `Workers` and follow `set up the Wrangler CLI`

3. Create KV storage:
```
wrangler kv:namespace create "CONFIG"
wrangler kv:namespace create "CONFIG" --preview
```

4. Rename `wrangler.toml.example` into `wrangler.toml` and fill parameters `name`, `account_id` and
`preview_id`, `id` in `kv_namespaces`
   
5. Create records in KV storage:
`botd_token` - authorization token, `botd_app` - origin backend url
   
6. Run `wrangler dev` for local testing

7. Run `wrangler publish` to deploy worker