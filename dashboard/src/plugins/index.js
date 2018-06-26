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

const router = new VueRouter({mode: 'history', routes: [].concat(forum).concat(survey).concat(nut)})

const store = new Vuex.Store({})

export default {
  router,
  store,
  render(h) {
    return h('router-view')
  }
}
