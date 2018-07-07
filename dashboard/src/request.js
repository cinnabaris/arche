import {GraphQLClient} from 'graphql-request'
import {notification} from 'antd'

export const client = new GraphQLClient('/graphql', {
  headers: {
    Authorization: 'Bearer my-jwt-token'
  },
  credentials: 'include',
  mode: 'cors'
})

export const failed = (err) => notification.error({
  // message: '',
  description: JSON.stringify(err.response),
  duration: 30
})

export const USERS_SIGN_UP = `mutation form($name: String!, $email: String!, $password: String!){
  signUpUser(name: $name, email: $email, password: $password) {
    createdAt
  }
}`

export const INSTALL = `mutation form($name: String!, $email: String!, $password: String!){
  install(name: $name, email: $email, password: $password) {
    createdAt
  }
}`

export const LIST_LOCALES_BY_LANG = `query locales($lang: String!){
  listLocalesByLang(lang: $lang) {
    code, message
  }
}`
