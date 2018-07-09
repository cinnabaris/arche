export const USERS_SIGN_IN = "users.sign-in"
export const USERS_SIGN_OUT = "users.sign-out"
export const PAGE_TITLE = "page.title"

export const signOut = () => {
  return {type: USERS_SIGN_OUT}
}

export const signIn = (token) => {
  return {type: USERS_SIGN_IN, token}
}

export const setPageTitle = (title) => {
  return {type: PAGE_TITLE, title}
}
