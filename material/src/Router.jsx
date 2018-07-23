import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  ConnectedRouter
} from 'connected-react-router'
import {
  Route,
  Switch
} from "react-router-dom"

import plugins from './plugins'
import NotFound from './plugins/NotFound'


class Widget extends Component {
  render() {
    const {
      history
    } = this.props
    return (<ConnectedRouter history={history}>
      <Switch>
        {plugins.routes.map((it)=><Route exact key={it.path} path={it.path} component={it.component} />)}
        <Route component={NotFound} />
      </Switch>
    </ConnectedRouter>)
  }
}

Widget.propTypes = {
  history: PropTypes.object.isRequired
}

export default Widget