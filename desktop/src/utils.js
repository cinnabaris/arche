import Cookies from 'js-cookie'

const TOKEN = 'token'

const LOCALE = 'locale'

export const getLocale = () => {
  return localStorage.getItem(LOCALE) || Cookies.get(LOCALE) || 'en-US'
}

export const setLocale = (lang) => {
  localStorage.setItem(LOCALE, lang)
  Cookies.set(LOCALE, lang, {
    expires: 1 << 16,
    path: '/'
  })
}
