import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {connect} from 'react-redux'

import UsersSignIn from './users/SignIn'
import UsersLogs from './users/Logs'

class Widget extends Component {
  render() {
    const {user} = this.props
    return user.uid
      ? <UsersLogs/>
      : <UsersSignIn/>
  }
}

Widget.propTypes = {
  user: PropTypes.object.isRequired
}

export default connect(state => ({user: state.currentUser}), {})(Widget)
