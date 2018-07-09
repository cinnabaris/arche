import enUSElement from 'element-ui/lib/locale/lang/en'
import zhHansElement from 'element-ui/lib/locale/lang/zh-CN'
import zhHantElement from 'element-ui/lib/locale/lang/zh-TW'
import 'dayjs/locale/zh-cn'
import 'dayjs/locale/zh-tw'
import Cookies from 'js-cookie'

import enUS from './en-US'
import zhHans from './zh-Hans'
import zhHant from './zh-Hant'

const KEY = 'locale'

export const set = (l) => {
  Cookies.set(KEY, l, {
    expires: 1 << 32 - 1
  });
  localStorage.setItem(KEY, l)
}

export const locales = {
  locale: Cookies.get(KEY) || localStorage.getItem(KEY) || 'en-US',
  fallbackLocale: 'en-US',
  messages: {
    'en-US': Object.assign({}, enUS, enUSElement),
    'zh-Hans': Object.assign({}, zhHans, zhHansElement),
    'zh-Hant': Object.assign({}, zhHant, zhHantElement)
  }
}
