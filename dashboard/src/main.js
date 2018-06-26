import 'vuetify/dist/vuetify.min.css'
import 'mdi/css/materialdesignicons.min.css'

import Vue from 'vue'
import Vuex from 'vuex'
import VueRouter from 'vue-router'
import VueI18n from 'vue-i18n'
import Vuetify from 'vuetify'

import routes from './plugins'
import messages from './locales'

Vue.config.productionTip = false

Vue.use(Vuex)
Vue.use(VueRouter)
Vue.use(VueI18n)
Vue.use(Vuetify)

const router = new VueRouter({base: '/my', mode: 'history', routes})
const store = new Vuex.Store({})
const i18n = new VueI18n({locale: 'en-US', messages})

new Vue({
  router,
  store,
  i18n,
  render(h) {
    return h('router-view')
  }
}).$mount('#app')
