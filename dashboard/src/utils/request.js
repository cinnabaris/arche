import {
  GraphQLClient
} from 'graphql-request'
import {
  notification
} from 'antd'
import moment from 'moment'

import {
  get as getToken
} from './token'

export const client = () => {
  return new GraphQLClient('/graphql', {
    headers: {
      Authorization: `Bearer ${getToken()}`
    },
    credentials: 'include',
    mode: 'cors'
  })
}

export const failed = (err) => notification.error({
  message: moment().format('ll LTS'),
  description: JSON.stringify(err.response),
  duration: 30
})