export const MEMBER = 'member'
export const ADMIN = 'admin'

const TOKEN = 'token'

export const get = () => {
  return localStorage.getItem(TOKEN)
}

export const set = (t) => {
  if (t) {
    localStorage.setItem(TOKEN, t)
  } else {
    localStorage.removeItem(TOKEN)
  }
}
