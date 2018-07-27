import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Col,
  Card
} from 'antd'
import {
  FormattedMessage
} from 'react-intl'

import Head from '../Head'

class Widget extends Component {
  render() {
    const {
      children,
      title
    } = this.props
    return (<Col xs={{
      span: 22,
      offset: 1
    }} lg={{
      span: 16,
      offset: 1
    }}>
      <Card title={(<FormattedMessage {...title}/>)}>
        {children}
        <Head title={title}/>
      </Card>
    </Col>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  title: PropTypes.object.isRequired
}

export default Widget