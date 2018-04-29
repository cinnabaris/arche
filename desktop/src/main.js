import Vue from 'vue'

import App from './App'
import plugins from './plugins'

Vue.config.productionTip = false

/* eslint-disable no-new */
new Vue({el: '#app',
  router: plugins.router,
  components: {
    App
  },
  template: '<App/>'})
