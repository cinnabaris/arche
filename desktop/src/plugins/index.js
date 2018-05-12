import Vue from 'vue'
import Router from 'vue-router'
import Vuex from 'vuex'

import Application from '@/layouts/application'
import Dashboard from '@/layouts/dashboard'
import nut from './nut'
import forum from './forum'

Vue.use(Router)
Vue.use(Vuex)

Vue.component('layout-application', Application)
Vue.component('layout-dashboard', Dashboard)

const plugins = [nut, forum]

export default {
  menus: plugins.reduce((ar, it) => ar.concat(it.menus), []),
  store: new Vuex.Store({state: {}, mutations: {}, actions: {}}),
  router: new Router({
    base: '/my/',
    mode: 'history',
    routes: plugins.reduce((ar, it) => ar.concat(it.routes), [])
  })
}
