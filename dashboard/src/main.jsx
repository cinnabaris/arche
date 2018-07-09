import React from 'react'
import ReactDOM from 'react-dom'
import {createStore, combineReducers, applyMiddleware} from 'redux'
import {Provider} from 'react-redux'
import createHistory from 'history/createBrowserHistory'
import {ConnectedRouter, routerReducer, routerMiddleware} from 'react-router-redux'
import {addLocaleData, IntlProvider} from 'react-intl'
import {Route} from "react-router"
import {LocaleProvider} from 'antd'

import './main.css';
import reducers from './reducers'
import {get as detectLocale} from './intl'
import {client, failed, LIST_LOCALES_BY_LANG} from './request'
import Layout from './layout'

const main = (id) => {
  const user = detectLocale()
  addLocaleData(user.data)

  const history = createHistory({basename: '/my'})
  const middleware = routerMiddleware(history)
  const store = createStore(combineReducers({
    ...reducers,
    router: routerReducer
  }), applyMiddleware(middleware))

  client.request(LIST_LOCALES_BY_LANG, {lang: user.locale}).then((rst) => {
    user.messages = rst.listLocalesByLang.reduce((ar, it) => {
      ar[it.code] = it.message
      return ar
    }, {})

    ReactDOM.render((<LocaleProvider locale={user.antd}>
      <IntlProvider locale={user.locale} messages={user.messages}>
        <Provider store={store}>
          <ConnectedRouter history={history}>
            <Route path="/" component={Layout}/>
          </ConnectedRouter>
        </Provider>
      </IntlProvider>
    </LocaleProvider>), document.getElementById(id))
  }).catch(failed)
}

export default main;
