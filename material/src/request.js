import {
  GraphQLClient
} from 'graphql-request'

import {
  getToken
} from './utils'

export const client = () => {
  return new GraphQLClient('/graphql', {
    headers: {
      Authorization: `Bearer ${getToken()}`
    },
    credentials: 'include',
    mode: 'cors'
  })
}