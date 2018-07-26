const KEY = 'token'

export const get = () => {
  return localStorage.getItem(KEY)
}

export const set = (token) => {
  localStorage.setItem(KEY, token)
}

export const remove = () => {
  localStorage.removeItem(KEY)
}