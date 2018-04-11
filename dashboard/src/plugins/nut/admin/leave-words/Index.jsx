import React, {Component} from 'react'
import {
  Row,
  Col,
  Table,
  Popconfirm,
  Button,
  message
} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import Moment from 'react-moment'

import Layout from '../../../../layouts/dashboard'
import {get, _delete} from '../../../../ajax'
import {ADMIN} from '../../../../auth'

class Widget extends Component {
  state = {
    items: []
  }
  componentDidMount() {
    get('/admin/leave-words').then((rst) => {
      this.setState({items: rst})
    }).catch(message.error);
  }
  handleRemove = (id) => {
    const {formatMessage} = this.props.intl
    _delete(`/admin/leave-words/${id}`).then((rst) => {
      message.success(formatMessage({id: 'flash.success'}))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({items})
    }).catch(message.error)
  }
  render() {
    const title = {
      id: "nut.admin.leave-words.index.title"
    }
    return (<Layout breads={[{
          href: "/admin/leave-words",
          label: title
        }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Table bordered={true} rowKey="id" dataSource={this.state.items} columns={[
              {
                title: <FormattedMessage id="attributes.created-at"/>,
                key: 'createdAt',
                render: (text, record) => (<Moment fromNow={true}>{record.createdAt}</Moment>)
              }, {
                title: <FormattedMessage id="attributes.content"/>,
                key: 'body',
                render: (text, record) => (<div>
                  <pre>{record.body}</pre>
                </div>)
              }, {
                title: <FormattedMessage id="attributes.action"/>,
                key: 'action',
                render: (text, record) => (<span>
                  <Popconfirm title={<FormattedMessage id = "helpers.are-you-sure" />} onConfirm={(e) => this.handleRemove(record.id)}>
                    <Button type="danger" shape="circle" icon="delete"/>
                  </Popconfirm>
                </span>)
              }
            ]}/>
        </Col>
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

const WidgetI = injectIntl(Widget)

export default connect(state => ({}), {},)(WidgetI)
