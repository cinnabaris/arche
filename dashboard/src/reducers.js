import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {USERS_SIGN_IN, PAGE_TITLE, USERS_SIGN_OUT} from './actions'
import {ADMIN, MEMBER, reload as reloadAuthorized} from './Authorized'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case USERS_SIGN_IN:
      try {
        var it = jwtDecode(action.token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          reloadAuthorized(
            it.admin
            ? ADMIN
            : MEMBER)
          return {uid: it.uid}
        }
      } catch (e) {
        console.error(e)
      }
      return {}
    case USERS_SIGN_OUT:
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
