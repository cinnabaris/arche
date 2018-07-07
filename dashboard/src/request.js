import {GraphQLClient} from 'graphql-request'

export const client = new GraphQLClient('/graphql', {
  headers: {
    Authorization: 'Bearer my-jwt-token'
  },
  credentials: 'include',
  mode: 'cors'
})

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
