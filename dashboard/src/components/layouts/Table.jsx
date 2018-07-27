import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Table,
  Col
} from 'antd'
import {
  FormattedMessage
} from 'react-intl'

import Head from '../Head'

class Widget extends Component {
  render() {
    const {
      items,
      title,
      columns
    } = this.props
    return (<Col xs={{span: 24}} lg={{span: 22, offset: 1}}>
      <Head title={title}/>
      <Table title={() => (<FormattedMessage {...title}/>)} bordered={true} rowKey="id" dataSource={items} columns={columns}/>
    </Col>);
  }
}

Widget.propTypes = {
  title: PropTypes.object.isRequired,
  items: PropTypes.array.isRequired,
  columns: PropTypes.array.isRequired
}

export default Widget