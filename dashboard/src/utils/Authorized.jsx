import RenderAuthorized from 'ant-design-pro/lib/Authorized'

import {
  client,
  failed
} from './request'

export const is_sign_in = (roles) => {
  console.log('must sign in', roles)
  return false
}

export const is_administrator = (roles) => {
  console.log('is administrator', roles)
  return false
}

export const is_forum_manager = (roles) => {
  console.log('is forum manager', roles)
  return false
}

export const is_caring_manager = (roles) => {
  console.log('is caring manager', roles)
  return false
}

let Authorized = RenderAuthorized('nil')

export const reload = (roles) => {
  client().request(`query info{
      listUserPolicy {
        role, resource
      }
    }`, {}).then((rst) => {
    Authorized = RenderAuthorized(JSON.stringify(rst.listUserPolicy))
  }).catch(failed)
}

export default Authorized