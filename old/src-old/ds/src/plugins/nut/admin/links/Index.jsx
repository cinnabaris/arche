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
import {push} from 'react-router-redux'

import Layout from '../../../../layouts/dashboard'
import {get, _delete} from '../../../../ajax'
import {ADMIN} from '../../../../auth'

class Widget extends Component {
  state = {
    items: []
  }
  componentDidMount() {
    get('/admin/links').then((rst) => {
      this.setState({items: rst})
    }).catch(message.error);
  }
  handleRemove = (id) => {
    const {formatMessage} = this.props.intl
    _delete(`/admin/links/${id}`).then((rst) => {
      message.success(formatMessage({id: 'flash.success'}))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({items})
    }).catch(message.error)
  }
  render() {
    const {push} = this.props
    const title = {
      id: "nut.admin.links.index.title"
    }
    return (<Layout breads={[{
          href: "/admin/links",
          label: title
        }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col>
          <Button onClick={(e) => push('/admin/links/new')} type='primary' shape="circle" icon="plus"/>
          <Table bordered={true} rowKey="id" dataSource={this.state.items} columns={[
              {
                title: <FormattedMessage id="attributes.loc"/>,
                key: 'loc',
                render: (text, record) => (<span>
                  {record.loc}({record.x},{record.y})
                </span>)
              }, {
                title: <FormattedMessage id="attributes.content"/>,
                dataIndex: 'label',
                render: (text, record) => (<a target="_blank" href={record.href}>{record.label}</a>)
              }, {
                title: <FormattedMessage id="attributes.action"/>,
                key: 'action',
                render: (text, record) => (<span>
                  <Button onClick={(e) => push(`/admin/links/edit/${record.id}`)} shape="circle" icon="edit"/>
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

export default connect(state => ({}), {
  push
},)(WidgetI)
