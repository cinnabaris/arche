export const SITE_REFRESH = 'site.refresh'
export const USERS_SIGN_IN = "users.sign-in"
export const USERS_SIGN_OUT = "users.sign-out"

export const TOKEN = "token"

export const refresh = (info) => {
  return {type: SITE_REFRESH, info}
}

export const signOut = () => {
  return {type: USERS_SIGN_OUT}
}

export const signIn = (token) => {
  return {type: USERS_SIGN_IN, token}
}
