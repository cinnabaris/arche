import React from 'react'

import Form from '../../../components/users/EmailForm'

const Widget = () => (<Form action="confirm" query={`query form($email: String!){
        confirmUser(email: $email) {
          createdAt
        }
      }`}/>)

export default Widget