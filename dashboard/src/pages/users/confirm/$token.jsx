import React from 'react'

import Form from '../../../components/users/TokenForm'

const Widget = () => (<Form action="confirm" query={`mutation form($token: String!){
        confirmUser(token: $token) {
          createdAt
        }
      }`}/>)

export default Widget