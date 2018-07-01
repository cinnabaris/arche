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
    'en-US': enUS,
    'zh-Hans': zhHans,
    'zh-Hant': zhHant
  }
}
