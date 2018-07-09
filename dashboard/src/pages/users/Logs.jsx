import React, {Component} from 'react'
import {Link} from "react-router-dom"

import Authorized, {MEMBER, ADMIN} from '../../Authorized'

class Widget extends Component {
  render() {
    return (<Authorized authority={[MEMBER, ADMIN]}>
      users.logs
      <Link to="/users/profile">Profile</Link>
    </Authorized>);
  }
}

export default Widget;
