import React from 'react'

import Form from '../../components/users/EmailForm'

const Widget = () => (<Form action="forgot-password" query={`query form($email: String!){
        forgotUserPassword(email: $email) {
          createdAt
        }
      }`}/>)

export default Widget