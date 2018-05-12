import Vue from 'vue'

import App from './App'
import plugins from './plugins'

Vue.config.productionTip = false

new Vue({
  router: plugins.router,
  store: plugins.store,
  render: (h) => h(App)
}).$mount('#app')
