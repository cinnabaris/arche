import React, {Component} from 'react'
import {
  Row,
  Col,
  Button,
  Collapse,
  Table,
  Popconfirm,
  message
} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import PropTypes from 'prop-types'
import SyntaxHighlighter from 'react-syntax-highlighter'
import {docco} from 'react-syntax-highlighter/styles/hljs'

import Layout from '../../../../layouts/dashboard'
import {get, put, _delete} from '../../../../ajax'
import {ADMIN} from '../../../../auth'

const Panel = Collapse.Panel

const Hash = ({item}) => (<Table rowKey="key" dataSource={Object.entries(item).map((v) => {
    return {key: v[0], val: v[1]}
  })} columns={[
    {
      title: <FormattedMessage id="attributes.key"/>,
      key: 'key',
      dataIndex: 'key'
    }, {
      title: <FormattedMessage id="attributes.value"/>,
      key: 'val',
      dataIndex: 'val'
    }
  ]}/>)

Hash.propTypes = {
  item: PropTypes.object.isRequired
}

class Widget extends Component {
  state = {
    os: {},
    postgresql: {},
    redis: "",
    routes: [],
    queue: {},
    network: {}
  }
  componentDidMount() {
    get('/admin/site/status').then((rst) => {
      this.setState(rst)
    }).catch(message.error);
  }
  handleClearCache = () => {
    const {formatMessage} = this.props.intl
    _delete('/admin/site/clear-cache').then((rst) => {
      message.success(formatMessage({id: 'flash.success'}))
    }).catch(message.error)
  }
  handleGeneratePages = () => {
    const {formatMessage} = this.props.intl
    put('/admin/site/generate-pages').then((rst) => {
      message.success(formatMessage({id: 'flash.success'}))
    }).catch(message.error)
  }
  render() {
    const {
      redis,
      os,
      network,
      postgresql,
      queue,
      routes
    } = this.state

    const title = {
      id: "nut.admin.site.status.title"
    }
    return (<Layout breads={[{
          href: "/admin/site/status",
          label: title
        }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 16,
            offset: 2
          }}>
          <Popconfirm title={<FormattedMessage id = "helpers.are-you-sure" />} onConfirm={this.handleClearCache}>
            <Button type="danger" icon="delete">
              <FormattedMessage id="nut.admin.site.status.clear-cache"/>
            </Button>
          </Popconfirm>
          <Popconfirm title={<FormattedMessage id = "helpers.are-you-sure" />} onConfirm={this.handleGeneratePages}>
            <Button type="dashed" icon="retweet">
              <FormattedMessage id="nut.admin.site.status.generate-pages"/>
            </Button>
          </Popconfirm>
        </Col>
        <Col md={{
            span: 16,
            offset: 2
          }}>
          <Collapse>
            <Panel key="os" header={(<FormattedMessage id="nut.admin.site.status.os"/>)}>
              <Hash item={os}/>
            </Panel>
            <Panel key="network" header={(<FormattedMessage id="nut.admin.site.status.network"/>)}>
              <Hash item={network}/>
            </Panel>
            <Panel key="postgresql" header={(<FormattedMessage id="nut.admin.site.status.postgresql"/>)}>
              <Hash item={postgresql}/>
            </Panel>
            <Panel key="queue" header={(<FormattedMessage id="nut.admin.site.status.queue"/>)}>
              <Hash item={queue}/>
            </Panel>
            <Panel key="redis" header={(<FormattedMessage id="nut.admin.site.status.redis"/>)}>
              <SyntaxHighlighter style={docco}>{redis}</SyntaxHighlighter>
            </Panel>
            <Panel header={(<FormattedMessage id='nut.admin.site.status.routes'/>)} key="routes">
              <Table rowKey={(r) => `${r.methods}-${r.path}`} dataSource={routes} columns={[
                  {
                    title: 'METHODS',
                    key: 'methods',
                    dataIndex: 'methods'
                  }, {
                    title: 'PATH',
                    key: 'path',
                    dataIndex: 'path'
                  }
                ]}/>
            </Panel>
          </Collapse>
        </Col>
      </Row>
    </Layout>);
  }
}
Widget.propTypes = {
  intl: intlShape.isRequired
}

const WidgetI = injectIntl(Widget)

export default WidgetI
