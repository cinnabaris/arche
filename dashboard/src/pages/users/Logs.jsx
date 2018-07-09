import React, {Component} from 'react'
import {Link} from "react-router-dom"

class Widget extends Component {
  render() {
    return (<div>
      users.logs
      <Link to="/users/profile">Profile</Link>
    </div>);
  }
}

export default Widget;
