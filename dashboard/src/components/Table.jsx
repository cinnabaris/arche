import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Table, Row} from 'antd'
import {FormattedMessage} from 'react-intl'

import Header from './Header'

class Widget extends Component {
  render() {
    const {items, title, columns} = this.props
    return (<Row>
      <Header title={title}/>
      <Table title={() => (<FormattedMessage {...title}/>)} bordered={true} rowKey="id" dataSource={items} columns={columns}/>
    </Row>);
  }
}

Widget.propTypes = {
  title: PropTypes.object.isRequired,
  items: PropTypes.array.isRequired,
  columns: PropTypes.array.isRequired
}

export default Widget;
