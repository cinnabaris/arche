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
    return (<Application title="nut.users.sign-in.title" submit={this.handleSubmit}>
      users.sign-in
    </Application>);
  }
}

export default Widget;