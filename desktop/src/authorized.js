export const CARING = 'caring'
export const FORUM = 'forum'
export const POS = 'pos'
export const HOTEL = 'hotel'
export const LIBRARY = 'library'

export const MANAGER = 'manager'
export const ADMIN = 'admin'

export const check = (user, role) => {
  // non-sign-in
  if (!user.uid || !user.policies) {
    return false
  }
  // only need sign-in
  if (!role) {
    return true
  }

  return user.policies.filter((it) => {
    if (it.roleName == ADMIN) {
      return true
    }
    return it.roleName === role.name && it.resourceType === role.type && it.resourceId === role.id
  }).length > 0
}
