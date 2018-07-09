import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Link} from "react-router-dom"
import {
  Row,
  Col,
  Card,
  Layout,
  Avatar,
  List,
  Menu,
  Icon
} from 'antd'
import DocumentTitle from 'react-document-title'

import Footer from './Footer'

const {Header, Content} = Layout;

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [
        {
          icon: 'login',
          label: 'nut.users.sign-in.title',
          to: '/users/sign-in'
        }, {
          icon: 'user-add',
          label: 'nut.users.sign-up.title',
          to: '/users/sign-up'
        }, {
          icon: 'retweet',
          label: 'nut.users.forgot-password.title',
          to: '/users/forgot-password'
        }, {
          icon: 'check',
          label: 'nut.users.confirm.title',
          to: '/users/confirm'
        }, {
          icon: 'unlock',
          label: 'nut.users.unlock.title',
          to: '/users/unlock'
        }, {
          icon: 'message',
          label: 'nut.leave-words.new.title',
          to: '/leave-words/new'
        }
      ]
    }
  }
  handleMenuClick = (e) => {
    switch (e.key) {
      case 'home':
        window.open("/", "_blank")
        break;
      default:
    }
  }
  render() {
    const {children, title} = this.props
    const {formatMessage} = this.props.intl
    return (<DocumentTitle title={`${formatMessage({
        id: title})} | ${formatMessage({id: 'site.title'})}`}>
      <Layout>
        <Content >
          <Header>
            <Menu onClick={this.handleMenuClick} theme="dark" mode="horizontal" style={{
                lineHeight: '64px'
              }}>
              <Menu.Item key="home"><FormattedMessage id="site.title"/></Menu.Item>
            </Menu>
          </Header>
          <Row>
            <Col xs={{
                span: 22,
                offset: 1
              }} lg={{
                span: 12,
                offset: 6
              }}>
              <br/>
              <Card title={<FormattedMessage id = {
                  title
                } />} extra={(<a href="/" target="_blank">
                  <Icon type="home"/></a>)}>
                {children}
              </Card>
              <br/>
              <List bordered={true} size="small" itemLayout="horizontal" dataSource={this.state.items} renderItem={it => (<List.Item>
                  <List.Item.Meta avatar={(<Avatar icon={it.icon}/>)} description={(<Link to={it.to
}>
                      <FormattedMessage id={it.label}/></Link>)}/>
                </List.Item>)}/>
              <br/>
            </Col>
          </Row>
        </Content>
        <Footer/>
      </Layout>
    </DocumentTitle>);
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  push: PropTypes.func.isRequired,
  intl: intlShape.isRequired,
  title: PropTypes.string.isRequired
}

export default connect(state => ({}), {push})(injectIntl(Widget))
