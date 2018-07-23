import {combineReducers} from 'redux'

import {SIGN_IN, SIGN_OUT, REFRESH} from './actions'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case SIGN_IN:
      // TODO parse token
      return [
        ...state, {
          id: 111
        }
      ]
    case SIGN_OUT:
      return {}
    default:
      return state
  }
}

const siteInfo = (state = {}, action) => {
  switch (action.type) {
    case REFRESH:
      return Object.assign({}, action.info)
    default:
      return state
  }
}

export default combineReducers({currentUser, siteInfo})
