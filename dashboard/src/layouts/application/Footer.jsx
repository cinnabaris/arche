import React, {Component} from 'react'
import {Layout} from 'antd'

const {Footer} = Layout

class Widget extends Component {

  render() {
    return (<Footer style={{
        textAlign: 'center'
      }}>
      application footer
    </Footer>);
  }
}

export default Widget;
