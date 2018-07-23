import dataEn from 'react-intl/locale-data/en'
import dataZh from 'react-intl/locale-data/zh'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'

import {
  getLocale
} from './utils'

export default () => {
  const lang = getLocale()

  switch (lang) {
    case 'zh-Hans':
      return {
        locale: lang,
        data: dataZh,
        moment: 'zh-cn'
      }
    case 'zh-Hant':
      return {
        locale: lang,
        data: dataZh,
        moment: 'zh-tw'
      }
    default:
      return {
        locale: 'en-US',
        data: dataEn,
        moment: null
      }
  }
}