import nut from './nut'
import caring from './caring'
import forum from './forum'

const plugins = [caring, forum, nut]

export const routes = plugins.reduce((ac, it) => {
  return ac.concat(it.routes)
}, [])

export const menus = plugins.reduce((ac, it) => {
  return ac.concat(it.menus)
}, [])