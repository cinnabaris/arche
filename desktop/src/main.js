import Vue from 'vue'
import VueI18n from 'vue-i18n'

import App from './App'
import plugins from './plugins'
import {get as getLocale} from './intl'
import {client} from './request'

Vue.config.productionTip = false
Vue.use(VueI18n)

let lng = getLocale()

client().request(`
  query getLocales($lang: String!){
    locales(lang: $lang){
      code,
      message
    }
  }
`, {lang: lng.locale}).then((rst) => {
  var messages = {}
  messages[lng.locale] = rst.locales.reduce(function (acc, cur) {
    acc[cur.code] = cur.message
    return acc
  }, {})

  const i18n = new VueI18n({
    locale: lng.locale, messages // set locale messages
  })

  /* eslint-disable no-new */
  new Vue({el: '#app',
    i18n,
    router: plugins.router,
    components: {
      App
    },
    template: '<App/>'})
})
