import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {USERS_SIGN_IN, USERS_SIGN_OUT, SITE_REFRESH, TOKEN} from './actions'

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

const siteInfo = (state = {
  languages: [],
  links: []
}, action) => {
  switch (action.type) {
    case SITE_REFRESH:
      // TODO using react-helmet
      // set title
      document.title = action.info.subhead + '|' + action.info.title
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

export default {currentUser, siteInfo}
