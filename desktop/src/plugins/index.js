import Vue from 'vue'
import Router from 'vue-router'
import Vuex from 'vuex'
import ElementUI from 'element-ui'
import VueQuillEditor from 'vue-quill-editor'

import 'element-ui/lib/theme-chalk/index.css'
import 'quill/dist/quill.core.css'
import 'quill/dist/quill.snow.css'
import 'quill/dist/quill.bubble.css'

import Application from '@/layouts/application'
import Dashboard from '@/layouts/dashboard'
import nut from './nut'
import forum from './forum'

Vue.use(Router)
Vue.use(Vuex)
Vue.use(ElementUI)
Vue.use(VueQuillEditor)

Vue.component('layout-application', Application)
Vue.component('layout-dashboard', Dashboard)

const plugins = [forum, nut]

export default {
  router: new Router({
    base: '/my',
    mode: 'history',
    routes: plugins.reduce((ar, it) => ar.concat(it.routes), [])
  })
}
