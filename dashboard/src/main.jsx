import './main.css'

import React from 'react'
import ReactDOM from 'react-dom'
import {
  Provider
} from 'react-redux'
import {
  createBrowserHistory
} from 'history'
import {
  applyMiddleware,
  compose,
  createStore
} from 'redux'
import {
  connectRouter,
  routerMiddleware
} from 'connected-react-router'
import {
  addLocaleData,
  IntlProvider
} from 'react-intl'

import reducers from './reducers'
import Router from './Router'
import detectLocale from './intl'
import {
  client,
  failed
} from './request'

const history = createBrowserHistory({
  basename: '/my/'
})

const store = createStore(
  connectRouter(history)(reducers), {},
  compose(
    applyMiddleware(routerMiddleware(history))
  )
)


export default (el) => {
  const user = detectLocale()
  addLocaleData(user.data)

  client().request(`query locales($lang: String!){
  listLocaleByLang(lang: $lang) {
    code, message
  }
}`, {
    lang: user.locale
  }).then((rst) => {
    user.messages = rst.listLocaleByLang.reduce((ar, it) => {
      ar[it.code] = it.message
      return ar
    }, {})

    ReactDOM.render((<Provider store={store}>
      <IntlProvider locale={user.locale} messages={user.messages}>
        <Router history={history}/>
      </IntlProvider>
    </Provider>), el)
  }).catch(failed)
}