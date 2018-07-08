import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {USERS_SIGN_IN, USERS_SIGN_OUT, TOKEN} from './actions'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case USERS_SIGN_IN:
      try {
        var it = jwtDecode(action.token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          return {admin: it.admin, uid: it.uid}
        }
        localStorage.removeItem(TOKEN);
      } catch (e) {
        console.error(e)
      }
      return {}
    case USERS_SIGN_OUT:
      localStorage.removeItem(TOKEN);
      return {}
    default:
      return state
  }
}

export default {currentUser}
