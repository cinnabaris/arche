import React, {Component} from 'react'
import Exception from 'ant-design-pro/lib/Exception'

import Layout from '../layouts/application'

class Widget extends Component {
  render() {
    return (<Layout breads={[]} title={{
        id: 'errors.not-found'
      }}><Exception type="404"/></Layout>)
  }
}

export default Widget
