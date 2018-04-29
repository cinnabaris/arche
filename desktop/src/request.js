import {GraphQLClient} from 'graphql-request'

import {get as getToken} from './auth'

export const client = () => new GraphQLClient(process.env.BACKEND, {
  mode: 'cors',
  credentials: 'include',
  headers: {
    Authorization: `Bearer ${getToken()}`
  }
})
