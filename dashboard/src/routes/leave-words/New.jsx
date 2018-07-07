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
    return (<Application title="nut.leave-words.new.title" submit={this.handleSubmit}>
      leave-words.new
    </Application>);
  }
}

export default Widget;