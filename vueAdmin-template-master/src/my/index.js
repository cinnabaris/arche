import Vue from 'vue'

import 'normalize.css/normalize.css'

import ElementUI from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
import locale from 'element-ui/lib/locale/lang/en'

import '@/styles/index.scss'

import App from '@/App'
import router from '@/router'
import store from '@/store'

import '@/icons'
import '@/permission'

Vue.use(ElementUI, { locale })

Vue.config.productionTip = false

const main = (id) => {
  new Vue({
    el: id,
    router,
    store,
    template: '<App/>', components: {
      App
    }})
}

export default main
