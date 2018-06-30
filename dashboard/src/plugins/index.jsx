import React from 'react'
import Loadable from 'react-loadable'
import {Route} from "react-router"

import nut from './nut'
import Loading from './Loading'
import NotFound from './NotFound'

const createLoading = (loader) => {
  return Loadable({loader: loader, loading: Loading})
}

const plugins = [nut]

export default {
  routes: plugins.reduce((ar, it) => ar.concat(it.routes), []).map((it) => (<Route key={it.name} exact={true} path={it.path} component={createLoading(it.component)}/>)).concat(<Route key="not-found" component={NotFound}/>),
  menus: []
}
