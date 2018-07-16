import 'material-design-icons-iconfont/dist/material-design-icons.css'
import 'element-ui/lib/theme-chalk/index.css'

import Vue from 'vue'
import ElementUI from 'element-ui'

import store from './store'
import i18n from './i18n'
import router from './plugins'
import './layouts'
import './components'

Vue.config.productionTip = false

Vue.use(ElementUI)

new Vue({
  i18n,
  store,
  router,
  render: h => h('router-view')
}).$mount('#app')
