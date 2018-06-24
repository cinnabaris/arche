import {GraphQLClient} from 'graphql-request'

import {getToken} from './auth'

export const client = () => new GraphQLClient(process.env.GRAPHQL_HOST, {
  mode: 'cors',
  credentials: 'include',
  headers: {
    Authorization: `Bearer ${getToken()}`
  }
})
