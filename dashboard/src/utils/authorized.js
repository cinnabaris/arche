import lodash from 'lodash'

export const is_sign_in = (authority) => {
  return authority && authority.length > 0
}

export const is_administrator = (authority) => {
  return authority && lodash.some(authority, {
    role: admin,
    resource: null
  })
}

const admin = 'admin'
const manager = 'manager'

export const FORUM = 'forum'
export const LIBRARY = 'library'
export const CARING = 'caring'
export const HOTEL = 'hotel'
export const POS = 'pos'

export const is_manager = (authority, resource) => {
  if (is_administrator(authority)) {
    return true
  }
  return lodash.some(authority, {
    role: manager,
    resource
  })
}