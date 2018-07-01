import 'vuetify/dist/vuetify.min.css'
import 'material-design-icons/iconfont/material-icons.css'
import '@mdi/font/css/materialdesignicons.css'

import Vue from 'vue'
import Vuex from 'vuex'
import VueRouter from 'vue-router'
import VueI18n from 'vue-i18n'
import Vuetify from 'vuetify'

import routes from './plugins'
import {locales} from './locales'
import './layouts'

Vue.config.productionTip = false

Vue.use(Vuex)
Vue.use(VueRouter)
Vue.use(VueI18n)
Vue.use(Vuetify)

const router = new VueRouter({base: '/my', mode: 'history', routes})
const store = new Vuex.Store({})
const i18n = new VueI18n(locales)

new Vue({
  router,
  store,
  i18n,
  render(h) {
    return h('router-view')
  }
}).$mount('#app')
