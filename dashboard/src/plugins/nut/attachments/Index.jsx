import React, {Component} from 'react'
import {
  Row,
  Col,
  Table,
  Popconfirm,
  Button,
  Upload,
  Icon,
  message
} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {CopyToClipboard} from 'react-copy-to-clipboard'

import Layout from '../../../layouts/dashboard'
import {get, _delete, backend} from '../../../ajax'
import {getToken, USER, ADMIN} from '../../../auth'

class Widget extends Component {
  state = {
    items: []
  }
  componentDidMount() {
    get('/attachments/').then((rst) => {
      this.setState({items: rst})
    }).catch(message.error);
  }
  handleRemove = (id) => {
    const {formatMessage} = this.props.intl
    _delete(`/attachments/${id}`).then((rst) => {
      message.success(formatMessage({id: 'flash.success'}))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({items})
    }).catch(message.error)
  }
  render() {
    const title = {
      id: "nut.attachments.index.title"
    }
    return (<Layout breads={[{
          href: "/attachments",
          label: title
        }
      ]} title={title} roles={[USER, ADMIN]}>
      <Row>
        <Col>
          <Upload multiple={true} withCredentials={true} name="file" action={backend("/attachments/")} headers={{
              'Authorization' : `BEARER ${getToken()}`
            }}>
            <Button>
              <Icon type="upload"/>
              <FormattedMessage id="buttons.upload"/>
            </Button>
          </Upload>
          <Table bordered={true} rowKey="id" dataSource={this.state.items} columns={[
              {
                title: <FormattedMessage id="attributes.content"/>,
                key: 'title',
                render: (text, record) => (<a href={record.url} target="_blank">
                  {record.title}
                </a>)
              }, {
                title: <FormattedMessage id="attributes.type"/>,
                dataIndex: 'mediaType',
                key: 'mediaType'
              }, {
                title: <FormattedMessage id="attributes.action"/>,
                key: 'action',
                render: (text, record) => (<span>
                  <CopyToClipboard text={record.url}><Button shape="circle" icon="copy"/></CopyToClipboard>
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
