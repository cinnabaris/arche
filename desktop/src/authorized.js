export const ADMIN = 'admin'

export const check = (user, role) => {
  if (!user || !user.uid || !user.groups) {
    return false
  }
  if (role) {
    if (user.groups.includes(ADMIN)) {
      return true
    }
    return user.groups.includes(role)
  }
  return true
}
