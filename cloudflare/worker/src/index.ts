import { Router } from 'itty-router'
import handleInitRequest from './handlers/init'
import handleAll from './handlers/all'

const router = Router()

router.get('/', handleInitRequest)
router.all('*', handleAll)

addEventListener('fetch', (e) => {
  e.respondWith(router.handle(e.request))
})
