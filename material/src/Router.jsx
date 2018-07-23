import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  ConnectedRouter
} from 'connected-react-router'

import Layout from './layout'

class Widget extends Component {
  render() {
    const {
      history
    } = this.props
    return (<ConnectedRouter history={history}>
      <Layout/>
    </ConnectedRouter>)
  }
}

Widget.propTypes = {
  history: PropTypes.object.isRequired
}

export default Widget