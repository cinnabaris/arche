import Cookies from 'js-cookie'
import dataEn from 'react-intl/locale-data/en'
import dataZh from 'react-intl/locale-data/zh'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import moment from 'moment'
import antdZhCn from 'antd/lib/locale-provider/zh_CN'
import antdZhTw from 'antd/lib/locale-provider/zh_TW'
import antdEnUs from 'antd/lib/locale-provider/en_US'

const KEY = 'locale'

export const get = () => {
  return localStorage.getItem(KEY) || Cookies.get(KEY) || 'en-US'
}

export const set = (lang) => {
  localStorage.setItem(KEY, lang)
  Cookies.set(KEY, lang, {
    expires: 1 << 16,
    path: '/'
  })
  window.location.reload()
}

export const detect = () => {
  const lang = get()
  switch (lang) {
    case 'zh-Hans':
      moment.locale('zh-cn')
      return {
        locale: lang,
        data: dataZh,
        antd: antdZhCn
      }
    case 'zh-Hant':
      moment.locale('zh-tw')
      return {
        locale: lang,
        data: dataZh,
        antd: antdZhTw
      }
    default:
      moment.locale()
      return {
        locale: 'en-US',
        data: dataEn,
        moment: null,
        antd: antdEnUs
      }
  }
}