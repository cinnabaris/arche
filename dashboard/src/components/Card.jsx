import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Row, Col, Card} from 'antd'
import {FormattedMessage} from 'react-intl'

import Header from './Header'

class Widget extends Component {
  render() {
    const {children, title} = this.props
    return (<Row>
      <Col xs={{
          span: 22,
          offset: 1
        }} lg={{
          span: 16,
          offset: 4
        }}>
        <Card title={(<FormattedMessage {...title}/>)}>
          {children}
          <Header title={title}/>
        </Card>
      </Col>
    </Row>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  title: PropTypes.object.isRequired
}

export default Widget
