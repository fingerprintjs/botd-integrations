import Config from './config'
import { insert } from './utils'
import { PATH_HASH } from './detect'

export function injectScript(html: string, config: Config): string {
  const scriptSrc = `https://openfpcdn.io/botd/v${config.version}/esm${config.debug ? ".min" : ""}.js`
  const script = `
    <script>
      function getResults() { 
        import('${scriptSrc}')
        .then(Botd => Botd.load({
          token: '${config.token}',
          endpoint: '${PATH_HASH}',
          mode: 'integration' 
        }))
        .then( detector => detector.detect() ) 
      }
      getResults()
    </script>`

  const match = /(<head.*>)/.exec(html)
  if (match === null) throw Error('Can`t find header tag in request body')

  return insert(html, match.index + match[0].length, script)
}
