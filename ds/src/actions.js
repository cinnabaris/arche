export const SITE_REFRESH = 'site.refresh'
export const USERS_SIGN_IN = "users.sign-in"
export const USERS_SIGN_OUT = "users.sign-out"
export const SIDE_BAR_TOGGLE = "side-bar.toggle"
export const SIDE_BAR_SELECT = "side-bar.select"

export const selectSideBar = (target) => {
  return {type: SIDE_BAR_SELECT, item: target}
}

export const toggleSideBar = (target) => {
  return {type: SIDE_BAR_TOGGLE, items: target}
}

export const refresh = (info) => {
  return {type: SITE_REFRESH, info}
}

export const signOut = () => {
  return {type: USERS_SIGN_OUT}
}

export const signIn = (token) => {
  return {type: USERS_SIGN_IN, token}
}
