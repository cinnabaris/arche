import Vue from 'vue'
import VueRouter from 'vue-router'

import nut from './nut'
import forum from './forum'

import NotFound from './NotFound'

Vue.use(VueRouter)

const routes = [].concat(nut).concat(forum).concat([
  {
    path: '*',
    component: NotFound
  }
])

const router = new VueRouter({mode: 'history', base: '/my/', routes})

export default router
