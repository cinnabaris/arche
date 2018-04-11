import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {USERS_SIGN_IN, USERS_SIGN_OUT, SITE_REFRESH, SIDE_BAR_TOGGLE, SIDE_BAR_SELECT} from './actions'
import {setToken, reloadAuthorized, ADMIN, USER} from './auth'

const sideBar = (state = {
  selected: [],
  open: []
}, action) => {
  switch (action.type) {
    case SIDE_BAR_SELECT:
      return Object.assign({}, state, {
        selected: [action.item]
      })
    case SIDE_BAR_TOGGLE:
      return Object.assign({}, state, {open: action.items})
    default:
      return state
  }
}

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case USERS_SIGN_IN:
      try {
        var it = jwtDecode(action.token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          reloadAuthorized(
            it.admin
            ? ADMIN
            : USER)
          setToken(action.token)
          return {uid: it.uid, admin: it.admin} // FIXME
        }
      } catch (e) {
        console.error(e)
      }
      reloadAuthorized()
      setToken()
      return {}
    case USERS_SIGN_OUT:
      reloadAuthorized()
      setToken()
      return {}
    default:
      return state
  }
}

const siteInfo = (state = {
  languages: []
}, action) => {
  switch (action.type) {
    case SITE_REFRESH:
      // set favicon
      var link = document.querySelector("link[rel*='icon']") || document.createElement('link');
      link.type = 'image/x-icon';
      link.rel = 'shortcut icon';
      link.href = action.info.favicon;
      document.getElementsByTagName('head')[0].appendChild(link);

      return Object.assign({}, action.info)
    default:
      return state;
  }
}

export default {currentUser, siteInfo, sideBar}
