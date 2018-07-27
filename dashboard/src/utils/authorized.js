import lodash from 'lodash'

export const is_sign_in = (authority) => {
  const roles = parse(authority)
  return roles.length > 0
}

export const is_administrator = (authority) => {
  const roles = parse(authority)
  console.log('is administrator', roles)
  return lodash.some(roles, {
    role: ''
  })
}

export const is_forum_manager = (authority) => {
  return is_manager(authority, 'forum')
}

export const is_caring_manager = (authority) => {
  return is_manager(authority, 'caring')
}

const admin = 'admin'
const manager = 'manager'

const is_manager = (authority, resource) => {
  const roles = parse(authority)
  if (lodash.some(roles, {
      role: admin,
      resource: null
    })) {
    return true
  }
  return lodash.some(roles, {
    role: manager,
    resource
  })
}

const parse = (authority) => {
  try {
    return JSON.parse(authority)
  } catch (e) {}
  return []
}