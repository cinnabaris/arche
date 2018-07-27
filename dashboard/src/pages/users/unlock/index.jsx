import React from 'react'

import Form from '../../../components/users/EmailForm'

const Widget = () => (<Form action="unlock" query={`query form($email: String!){
        unlockUser(email: $email) {
          createdAt
        }
      }`}/>)

export default Widget