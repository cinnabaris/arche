import 'element-ui/lib/theme-chalk/index.css';

import Vue from 'vue'
import Vuex from 'vuex'
import VueRouter from 'vue-router'
import VueI18n from 'vue-i18n'
import ElementUI from 'element-ui';

import routes from './plugins'
import {locales} from './locales'
import './layouts'

Vue.config.productionTip = false

Vue.use(Vuex)
Vue.use(VueRouter)
Vue.use(VueI18n)

const router = new VueRouter({base: '/my', mode: 'history', routes})
const store = new Vuex.Store({})
const i18n = new VueI18n(locales)

Vue.use(ElementUI, {
  i18n: (key, val) => i18n.t(key, val)
})

new Vue({
  router,
  store,
  i18n,
  render(h) {
    return h('router-view')
  }
}).$mount('#app')
