import {
  combineReducers
} from 'redux'
import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {
  SIGN_IN,
  SIGN_OUT
} from './actions'
import {
  setToken,
  removeToken
} from './utils'
import {
  reload as reloadAuthorized
} from './Authorized'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case SIGN_IN:
      try {
        const token = action.token
        var it = jwtDecode(token)
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          setToken(token)
          reloadAuthorized()
          return {
            uid: it.uid
          }
        }
      } catch (e) {
        // eslint-disable-next-line
        console.error(e)
      }
      removeToken()
      return {}
    case SIGN_OUT:
      removeToken()
      return {}
    default:
      return state
  }
}

export default combineReducers({
  currentUser
})