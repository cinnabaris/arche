import {GraphQLClient} from 'graphql-request'

import {getToken} from './auth'

export const client = () => new GraphQLClient(process.env.REACT_APP_BACKEND, {
  mode: 'cors',
  credentials: 'include',
  headers: {
    Authorization: `Bearer ${getToken()}`
  }
})
