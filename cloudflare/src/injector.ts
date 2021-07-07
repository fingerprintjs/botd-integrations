import Config from './config'
import { BOTD_DEFAULT_PATH } from './constants'

export function injectScript(html: string, config: Config): string {
  const script = `<script>
        async function getResults() {
            const botdPromise = Botd.load({
                token: "${config.token}",
                mode: "requestId",
                endpoint: "${config.botdURL + BOTD_DEFAULT_PATH}",
            })
            const botd = await botdPromise
            const result = await botd.detect("Cloudflare")
        }
    </script>
    <script src="https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0.1.14/dist/botd.min.js" onload="getResults()"></script>`

  const match = /(<head.*>)/.exec(html)
  if (match === null) throw Error('Can`t find header tag in request body')

  return html.substr(0, match.index) + script + html.substr(match.index)
}
