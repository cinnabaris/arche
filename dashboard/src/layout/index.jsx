import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  injectIntl,
  intlShape
} from 'react-intl'
import {
  connect
} from 'react-redux'
import {
  push
} from 'connected-react-router'
import {
  Route,
  Switch
} from "react-router-dom"
import Exception from 'ant-design-pro/lib/Exception'
import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import {
  Icon,
  Layout,
  Menu,
  message,
  Modal
} from 'antd'

import {
  signIn,
  signOut
} from '../actions'
import plugins from '../plugins'
import createLoading from '../loading'

const {
  Header,
  Sider,
  Content
} = Layout

class Widget extends Component {
  render() {
    return (<div>
      <div>header</div>
      <Switch>
        {plugins.routes.map((it)=><Route exact key={it.path} path={it.path} component={createLoading(it.component)} />)}
        <Route component={()=><Exception type="404" />} />
      </Switch>
      <div>footer</div>
    </div>)
  }
}

Widget.propTypes = {
  push: PropTypes.func.isRequired,
  intl: intlShape.isRequired,
  user: PropTypes.object.isRequired,
  signIn: PropTypes.func.isRequired,
  signOut: PropTypes.func.isRequired,
}

export default connect(state => ({
  user: state.currentUser
}), {
  push,
  signIn,
  signOut
})(injectIntl(Widget))