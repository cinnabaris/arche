import nut from './nut'

const plugins = [nut]

const routes = plugins.reduce((ac, it) => {
  return ac.concat(it.routes)
}, [])

export default {
  routes
}