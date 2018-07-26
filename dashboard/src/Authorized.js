import RenderAuthorized from 'ant-design-pro/lib/Authorized'

import {
  client
} from './request'

export const is_sign_in = (roles) => {
  console.log(roles)
  return false
}

export const is_administrator = (roles) => {
  console.log(roles)
  return false
}

export const is_forum_manager = (roles) => {
  console.log(roles)
  return false
}

export const is_caring_manager = (roles) => {
  console.log(roles)
  return false
}

let Authorized = RenderAuthorized('nil')

export const reload = (roles) => {
  client().request(`query info{
      listUserPolicy {
        role, resource
      }
    }`, {}).then((rst) => {
    // TODO to roles string
    console.log(rst.listUserPolicy)
    // this.$store.commit('updatePolicies', rst.listUserPolicy)
    // this.auth()
    // Authorized = RenderAuthorized(roles)
  }).catch(console.error)
}

export default Authorized