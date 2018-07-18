import Vue from 'vue'
import VueI18n from 'vue-i18n'
import moment from 'moment'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'

import {getLocale} from './utils'
import messages from './locales'

Vue.use(VueI18n)

const locale = getLocale()

switch (locale) {
  case 'zh-Hans':
    moment.locale('zh-cn')
    break
  case 'zh-Hant':
    moment.locale('zh-tw')
    break
  default:
    break
}

const i18n = new VueI18n({locale, messages})

export default i18n
