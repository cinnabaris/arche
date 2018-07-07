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
    return (<Application title="nut.users.reset-password.title" submit={this.handleSubmit}>
      users.reset-password
    </Application>);
  }
}

export default Widget;