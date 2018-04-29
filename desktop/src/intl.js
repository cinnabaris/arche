import Cookies from 'js-cookie'

const KEY = 'locale'

export const set = (lang) => {
  localStorage.setItem(KEY, lang)
  Cookies.set(KEY, lang, {
    expires: 365 * 100,
    path: '/'
  })
}

export const get = () => {
  var lang = localStorage.getItem(KEY) || Cookies.get(KEY)
  switch (lang) {
    case 'zh-Hans':
      return {locale: lang, moment: 'zh-cn'}
    case 'zh-Hant':
      return {locale: lang, moment: 'zh-tw'}
    default:
      return {locale: 'en-US', moment: 'en-gb'}
  }
}
