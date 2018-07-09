import React, {Component} from 'react'
import PropTypes from 'prop-types'
import RenderAuthorized from 'ant-design-pro/lib/Authorized'
import Exception from 'ant-design-pro/lib/Exception'
import jwtDecode from 'jwt-decode'
import moment from 'moment'

export const ADMIN = 'admin'
export const MEMBER = 'member'
export const NULL = 'null'

export const TOKEN = "token"

let Authorized = RenderAuthorized('nil')

const parse = (token) => {
  if (token) {
    try {
      var it = jwtDecode(token);
      if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
        return it.admin
          ? ADMIN
          : MEMBER
      }
    } catch (e) {
      console.log(e)
    }
  }
  return NULL
}

export const reload = (token) => {
  Authorized = RenderAuthorized(parse(token))
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