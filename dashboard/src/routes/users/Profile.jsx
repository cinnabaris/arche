import React, {Component} from 'react'
import {Link} from "react-router-dom"

class Widget extends Component {
  render() {
    return (<div>
      users.profile
      <Link to="/dashboard/users/logs">logs</Link>
    </div>);
  }
}

export default Widget;
