import Vue from 'vue'
import Vuex from 'vuex'
import VueRouter from 'vue-router'
import VueI18n from 'vue-i18n'
import Vuetify from 'vuetify'

import nut from './nut'
import survey from './survey'
import forum from './forum'

Vue.use(Vuex)
Vue.use(VueRouter)
Vue.use(VueI18n)
Vue.use(Vuetify)

const routes = [].concat(forum).concat(survey).concat(nut)
const router = new VueRouter({base: '/my', mode: 'history', routes})
const store = new Vuex.Store({})

export default {
  router,
  store,
  render(h) {
    return h('router-view')
  }
}
