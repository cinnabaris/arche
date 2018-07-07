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
    return (<Application title="nut.users.confirm.title" submit={this.handleSubmit}>
      users.confirm
    </Application>);
  }
}

export default Widget;