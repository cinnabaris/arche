import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {USERS_SIGN_IN, PAGE_TITLE, USERS_SIGN_OUT} from './actions'
import {TOKEN} from './Authorized'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case USERS_SIGN_IN:
      try {
        var it = jwtDecode(action.token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          localStorage.setItem(TOKEN, action.token)
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

const pageTitle = (state = {}, action) => {
  switch (action.type) {
    case PAGE_TITLE:
      return {title: action.title}
    default:
      return state
  }
}

export default {currentUser, pageTitle}
