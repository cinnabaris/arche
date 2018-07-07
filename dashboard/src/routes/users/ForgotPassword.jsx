import React, {
  Component
} from 'react'

import Application from '../../layouts/Application'

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault();
    console.log('aaa')
  }
  render() {
    return (<Application title="nut.users.forgot-password.title" submit={this.handleSubmit}>
      users.forgot-password
    </Application>);
  }
}

export default Widget;