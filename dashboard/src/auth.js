import RenderAuthorized from 'ant-design-pro/lib/Authorized';

export const USER = 'user'
export const ADMIN = 'admin'

const TOKEN = "token"

export let Authorized = RenderAuthorized();

export const getToken = () => {
  return localStorage.getItem(TOKEN)
}

export const setToken = (t) => {
  if (t) {
    localStorage.setItem(TOKEN, t)
  } else {
    localStorage.removeItem(TOKEN);
  }
}

export const reloadAuthorized = (role) => {
  Authorized = RenderAuthorized(role)
}
