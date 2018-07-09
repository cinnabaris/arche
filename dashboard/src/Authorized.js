import React, {Component} from 'react'
import PropTypes from 'prop-types'
import RenderAuthorized from 'ant-design-pro/lib/Authorized'
import Exception from 'ant-design-pro/lib/Exception'

export const ADMIN = 'admin'
export const MEMBER = 'member'
export const NULL = 'null'

export const TOKEN = "token"

let Authorized = RenderAuthorized('nil')

export const reload = (role) => {
  Authorized = RenderAuthorized(role)
}

class Widget extends Component {
  render() {
    const {children, authority} = this.props
    return (<Authorized authority={authority} noMatch={(<Exception type="403"/>)}>
      {children}
    </Authorized>);
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  authority: PropTypes.arrayOf(PropTypes.string).isRequired
}
export default Widget;

// export default Authorized