import Config from './config'
import { insert } from './utils'
import { PATH_HASH } from './detect'

export function injectScript(html: string, config: Config): string {
  const script = `<script>
    function getResults(){
      Botd.load({
        token: "${config.token}",
        endpoint: "${PATH_HASH}",
        isIntegration: true
      }).then( b => b.detect() )
    }
    </script>
    <script src="https://cdn.jsdelivr.net/npm/@fpjs-incubator/botd-agent@0/dist/botd.min.js" onload="getResults()"></script>`

  const match = /(<head.*>)/.exec(html)
  if (match === null) throw Error('Can`t find header tag in request body')

  return insert(html, match.index, script)
}
