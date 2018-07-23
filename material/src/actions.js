export const REFRESH = 'refresh'
export const SIGN_IN = 'users.sign-in'
export const SIGN_OUT = 'users.sign-out'

export const refresh = (info) => ({type: REFRESH, info})
export const signIn = (token) => ({type: SIGN_IN, token})
export const signOut = () => ({type: SIGN_OUT})
