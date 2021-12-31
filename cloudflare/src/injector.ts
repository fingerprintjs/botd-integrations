import Config from './config'
import { PATH_HASH } from './detect'

class ElementHandler {
  script: string

  constructor(script: string) {
    this.script = script
  }

  element(element: Element) {
    element.append(this.script, { html: true })
  }
}

export function injectScript(resp: Response, config: Config): Response {
  const scriptSrc = `https://openfpcdn.io/botd/v${config.version}/esm${config.debug ? '.min' : ''}.js`
  const script = `<script>
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
    </script>
`
  return new HTMLRewriter().on('head', new ElementHandler(script)).transform(resp)
}
