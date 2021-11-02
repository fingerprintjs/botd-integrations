import Config from './config'
import { insert } from './utils'
import { PATH_HASH } from './detect'

export function injectScript(html: string, config: Config): string {
  const scriptSrc = `https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@${config.version == 'latest' ? "0" : config.version}/dist/botd${config.debug ? ".min" : ""}.js`

  const script = `<script>
    function getResults(){
      Botd.load({
        token: "${config.token}",
        endpoint: "${PATH_HASH}",
        mode: "integration"
      }).then( b => b.detect() )
    }
    </script>
    <script src="${scriptSrc}" onload="getResults()"></script>`

  const match = /(<head.*>)/.exec(html)
  if (match === null) throw Error('Can`t find header tag in request body')

  return insert(html, match.index, script)
}
