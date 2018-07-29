import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  connect
} from 'dva'
import Exception from 'ant-design-pro/lib/Exception'

class Widget extends Component {
  render() {
    const {
      currentUser,
      children,
      check,
      hidden
    } = this.props
    if (check(currentUser)) {
      return children
    }
    if (hidden) {
      return (<span/>)
    }
    return (<Exception type="403" />);
  }
}

Widget.propTypes = {
  check: PropTypes.func.isRequired,
  currentUser: PropTypes.object.isRequired,
  children: PropTypes.node.isRequired,
  hidden: PropTypes.bool,
}

export default connect(({
  currentUser
}) => ({
  currentUser,
}))(Widget)