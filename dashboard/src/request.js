import {GraphQLClient} from 'graphql-request'
import {notification} from 'antd'
import moment from 'moment'

import {TOKEN} from './actions'

export const USERS_SIGN_IN = `mutation form($email: String!, $password: String!){
  signInUserByEmail(email: $email, password: $password) {
    token
  }
}`

export const USERS_RESET_PASSWORD = `mutation form($token: String!, $password: String!){
  resetUserPassword(token: $token, password: $password) {
    createdAt
  }
}`

export const USERS_UNLOCK_TOKEN = `mutation form($token: String!){
  unlockUser(token: $token) {
    createdAt
  }
}`

export const USERS_CONFIRM_TOKEN = `mutation form($token: String!){
  confirmUser(token: $token) {
    createdAt
  }
}`

export const USERS_FORGOT_PASSWORD = `query form($email: String!){
  forgotUserPassword(email: $email) {
    createdAt
  }
}`

export const USERS_UNLOCK = `query form($email: String!){
  unlockUser(email: $email) {
    createdAt
  }
}`

export const USERS_CONFIRM = `query form($email: String!){
  confirmUser(email: $email) {
    createdAt
  }
}`

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

export const client = new GraphQLClient('/graphql', {
  headers: {
    Authorization: `Bearer ${localStorage.getItem(TOKEN)}`
  },
  credentials: 'include',
  mode: 'cors'
})

export const failed = (err) => notification.error({
  message: moment().format('ll LTS'),
  description: JSON.stringify(err.response),
  duration: 30
})
