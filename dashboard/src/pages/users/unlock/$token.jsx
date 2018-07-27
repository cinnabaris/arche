import React from 'react'

import Form from '../../../components/users/TokenForm'

const Widget = () => (<Form action="unlock" query={`mutation form($token: String!){
        unlockUser(token: $token) {
          createdAt
        }
      }`}/>)

export default Widget