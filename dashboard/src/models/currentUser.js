import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {
  set as setToken,
  remove as removeToken
} from '../utils/token'

export default {
  namespace: 'currentUser',
  state: {},
  reducers: {
    'refresh' (state, payload) {
      return Object.assign(state, {
        authority: payload.authority
      })
    },
    'sign-in' (state, payload) {
      const token = payload.token
      try {
        var it = jwtDecode(token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          setToken(token)
          return {
            uid: it.uid
          }
        }
        return
      } catch (e) {
        // eslint-disable-next-line
        console.error(e)
      }
      removeToken()
      return {}
    },
    'sign-out' (state) {
      removeToken()
      return {}
    }
  },
};