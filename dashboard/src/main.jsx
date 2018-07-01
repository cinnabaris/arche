import React from 'react'
import ReactDOM from 'react-dom'
import {Route} from "react-router"
import {createStore, combineReducers, applyMiddleware} from 'redux'
import {Provider} from 'react-redux'
import createHistory from 'history/createBrowserHistory'
import {Switch} from 'react-router-dom'
import {ConnectedRouter, routerReducer, routerMiddleware} from 'react-router-redux'
import {addLocaleData, IntlProvider} from 'react-intl'
import {LocaleProvider} from 'antd'
import {request} from 'graphql-request'

import './main.css';
import Application from './layouts/application'
import Dashboard from './layouts/dashboard'
import NotFound from './routes/NotFound'
import reducers from './reducers'
import {get as detectLocale} from './intl'
import {BACKEND} from './config'

import Home from './routes/Home'

const main = (id) => {
  const user = detectLocale()
  addLocaleData(user.data)

  const history = createHistory({basename: '/my'})
  const middleware = routerMiddleware(history)
  const store = createStore(combineReducers({
    ...reducers,
    router: routerReducer
  }), applyMiddleware(middleware))

  request(BACKEND, `
{
  listLocalesByLang(lang:"${user.locale}") {
    id
  }
}
  `).then((rst) => {
    user.messages = {
      ...rst
    }

    ReactDOM.render((<LocaleProvider locale={user.antd}>
      <IntlProvider locale={user.locale} messages={user.messages}>
        <Provider store={store}>
          <ConnectedRouter history={history}>
            <Switch>
              <Route exact={true} component={Application}/>
              <Route exact={true} component={Dashboard}/>
              <Route exact={true} path="/" component={Home}/>
              <Route component={NotFound}/>
            </Switch>
          </ConnectedRouter>
        </Provider>
      </IntlProvider>
    </LocaleProvider>), document.getElementById(id))
  })
}

export default main;
