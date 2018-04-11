import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Row, Col, Table, message} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'

import Layout from '../../../layouts/dashboard'
import {get} from '../../../ajax'
import TimeAgo from '../../../components/TimeAgo'
import {USER, ADMIN} from '../../../auth'

class Widget extends Component {
  state = {
    items: []
  }
  componentDidMount() {
    get('/users/logs').then((rst) => {
      this.setState({items: rst})
    }).catch(message.error);
  }
  render() {
    const title = {
      id: "nut.users.logs.title"
    }
    return (<Layout breads={[{
          href: "/users/logs",
          label: title
        }
      ]} roles={[USER, ADMIN]} title={title}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Table bordered={true} rowKey="id" dataSource={this.state.items} columns={[
              {
                title: <FormattedMessage id="attributes.created-at"/>,
                key: 'createdAt',
                render: (text, record) => (<TimeAgo value={record.createdAt}/>)
              }, {
                title: <FormattedMessage id="attributes.ip"/>,
                dataIndex: 'ip',
                key: 'ip'
              }, {
                title: <FormattedMessage id="nut.attributes.log.message"/>,
                dataIndex: 'message',
                key: 'message'
              }
            ]}/>
        </Col>
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  user: PropTypes.object.isRequired
}

const WidgetI = injectIntl(Widget)

export default connect(state => ({user: state.currentUser}), {},)(WidgetI)
