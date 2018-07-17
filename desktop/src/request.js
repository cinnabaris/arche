import {GraphQLClient} from 'graphql-request'
import moment from 'moment'
import {Notification} from 'element-ui'

import {getToken} from './utils'

export const client = () => {
  return new GraphQLClient('/graphql', {
    headers: {
      Authorization: `Bearer ${getToken()}`
    },
    credentials: 'include',
    mode: 'cors'
  })
}

export const failed = (err) => Notification.error({
  title: moment().format('ll LTS'),
  message: JSON.stringify(err.response),
  duration: 30 * 1000
})
