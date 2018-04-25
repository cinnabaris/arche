import {getToken} from './auth'

export const backend = (path) => {
  return process.env.REACT_APP_BACKEND + path;
}

const parse = (res) => {
  // res.status === 200 || res.status === 0
  return res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    })
}

export const options = (method) => {
  return {
    method: method,
    mode: 'cors',
    credentials: 'include',
    headers: {
      'Authorization': `Bearer ${getToken()}`
    }
  }
}

export const get = (path) => {
  return fetch(backend(path), options('GET')).then(parse)
}

export const _delete = (path) => {
  return fetch(backend(path), options('DELETE')).then(parse)
}

// https://github.github.io/fetch/#options
export const post = (path, body) => {
  var data = options('POST')
  data.body = JSON.stringify(body)
  data.headers['Content-Type'] = 'application/json'
  return fetch(backend(path), data).then(parse)
}

export const patch = (path, body) => {
  var data = options('PATCH')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then(parse)
}

export const put = (path, body) => {
  var data = options('PUT')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then(parse)
}
