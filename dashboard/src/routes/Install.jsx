import React, {
  Component
} from 'react'

import Application from '../layouts/Application'

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault();
    console.log('aaa')
  }
  render() {
    return (<Application title="install.title" submit={this.handleSubmit}>
      install
    </Application>);
  }
}

export default Widget;