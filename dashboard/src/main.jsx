import React from 'react'
import ReactDOM from 'react-dom'
import {createStore, compose, applyMiddleware} from 'redux'
import {Provider} from 'react-redux'
import {createBrowserHistory} from 'history'
import {connectRouter, routerMiddleware, ConnectedRouter} from 'connected-react-router'
import {addLocaleData, IntlProvider} from 'react-intl'
import {Route} from "react-router"
import {LocaleProvider} from 'antd'
import 'moment-timezone'

import './main.css';
import reducers from './reducers'
import {get as detectLocale} from './intl'
import {client, failed, LIST_LOCALES_BY_LANG} from './request'
import Layout from './layout'

const main = (id) => {
  const user = detectLocale()
  addLocaleData(user.data)

  const history = createBrowserHistory({basename: '/my/'})
  // const middleware = routerMiddleware(history)
  // const store = createStore(combineReducers({
  //   ...reducers
  // }), applyMiddleware(middleware))
  const store = createStore(connectRouter(history)(...reducers), {}, compose(applyMiddleware(routerMiddleware(history))))

  client().request(LIST_LOCALES_BY_LANG, {lang: user.locale}).then((rst) => {
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
