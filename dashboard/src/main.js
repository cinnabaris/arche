import React from 'react'
import ReactDOM from 'react-dom'
import {createStore, combineReducers, applyMiddleware} from 'redux'
import {Provider} from 'react-redux'
import createHistory from 'history/createBrowserHistory'
import {Switch} from 'react-router-dom'
import {ConnectedRouter, routerReducer, routerMiddleware} from 'react-router-redux'
import {addLocaleData, IntlProvider} from 'react-intl'
import {LocaleProvider} from 'antd'

import './main.css';
import reducers from './reducers'
import plugins from './plugins'
import {get as detectLocale} from './intl'
import {client} from './request'

const main = (id) => {
  const user = detectLocale()
  addLocaleData(user.data)

  const history = createHistory({basename: '/my'})
  const middleware = routerMiddleware(history)
  const store = createStore(combineReducers({
    ...reducers,
    router: routerReducer
  }), applyMiddleware(middleware))

  client().request(`
    query getLocales($lang: String!){
      locales(lang: $lang){
        code,
        message
      }
    }
  `, {lang: user.locale}).then((rst) => {
    var messages = rst.locales.reduce(function(acc, cur) {
      acc[cur.code] = cur.message
      return acc
    }, {})
    user.messages = {
      ...messages
    }
    ReactDOM.render((<LocaleProvider locale={user.antd}>
      <IntlProvider locale={user.locale} messages={user.messages}>
        <Provider store={store}>
          <ConnectedRouter history={history}>
            <Switch>
              {plugins.routes}
            </Switch>
          </ConnectedRouter>
        </Provider>
      </IntlProvider>
    </LocaleProvider>), document.getElementById('root'))
  })
}

export default main;
