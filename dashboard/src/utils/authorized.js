import lodash from 'lodash'

export const is_sign_in = (user) => {
  return user && user.uid && user.authority
}

export const is_administrator = (user) => {
  if (!is_sign_in(user)) {
    return false
  }
  return lodash.some(user.authority, {
    role: ADMIN,
    resource: null
  })
}

const ADMIN = 'admin'
export const MANAGER = 'manager'

export const FORUM = 'forum'
export const LIBRARY = 'library'
export const CARING = 'caring'
export const HOTEL = 'hotel'
export const POS = 'pos'


export const is_forum_manager = (user) => {
  return is_manager(user, FORUM)
}

export const is_library_manager = (user) => {
  return is_manager(user, LIBRARY)
}

export const is_pos_manager = (user) => {
  return is_manager(user, POS)
}

export const is_caring_manager = (user) => {
  return is_manager(user, CARING)
}

export const is_hotel_manager = (user) => {
  return is_manager(user, HOTEL)
}

const is_manager = (user, resource) => {
  if (is_administrator(user.authority)) {
    return true
  }
  return lodash.some(user.authority, {
    role: MANAGER,
    resource
  })
}