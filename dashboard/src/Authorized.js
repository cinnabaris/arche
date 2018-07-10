import RenderAuthorized from 'ant-design-pro/lib/Authorized'

export const ADMIN = 'admin'
export const MEMBER = 'member'
export const NULL = 'null'

export const ALL = [ADMIN, MEMBER]
export const TOKEN = "token"

let Authorized = RenderAuthorized(NULL)

export const reload = (role) => {
  Authorized = RenderAuthorized(role)
}

export default Authorized
