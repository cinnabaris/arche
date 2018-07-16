import Vue from 'vue'
import VueRouter from 'vue-router'

import nut from './nut'
import forum from './forum'

Vue.use(VueRouter)

const routes = [].concat(nut).concat(forum)

const router = new VueRouter({mode: 'history', base: '/my/', routes})

export default router
