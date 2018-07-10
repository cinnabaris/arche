import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import Exception from 'ant-design-pro/lib/Exception'
import {Route} from "react-router"
import {Icon, Layout, Menu, message, Modal} from 'antd'
import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import {Switch} from 'react-router-dom'

import createLoading from '../loading'
import {routes, menus} from '../router'
import Authorized, {ALL, TOKEN} from '../Authorized'
import {signIn, signOut} from '../actions'
import {client, USERS_SIGN_OUT, failed} from '../request'
import NoticeBar from './NoticeBar'
import Footer from './Footer'

const {Header, Sider, Content} = Layout

const show = (it) => {
  let Children = createLoading(it.component)
  if (it.authority) {
    return() => (<Authorized authority={it.authority} noMatch={(<Exception type="403"/>)}><Children/></Authorized>)
  }
  return Children
}

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      title: null,
      collapsed: false
    }
  }
  handleSiderClick = (e) => {
    const {push} = this.props
    push(e.key)
  }
  handleHeaderClick = (e) => {
    const {push, signOut} = this.props
    const {formatMessage} = this.props.intl
    switch (e.key) {
      case 'home':
        window.open("/", "_blank")
        return;
      case 'doc':
        window.open('https://github.com/cinnabaris/arche', "_blank")
        return
      case 'toggle':
        this.setState({
          collapsed: !this.state.collapsed
        })
        return
      case 'sign-out':
        Modal.confirm({
          title: formatMessage({id: "helpers.are-you-sure"}),
          onOk() {
            client().request(USERS_SIGN_OUT, {}).then((rst) => {
              push('/users/sign-in')
              message.info(formatMessage({id: "flashes.success"}))
              localStorage.removeItem(TOKEN)
              signOut()
            }).catch(failed)
          }
        })
        return
      default:
        console.log(e.key)
    }
  }
  componentDidMount() {
    const {user, signIn} = this.props
    if (!user.uid) {
      let token = localStorage.getItem(TOKEN)
      if (token) {
        signIn(token)
      }
    }
  }
  render() {
    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" trigger={null} collapsible="collapsible" collapsed={this.state.collapsed}>
        <div className="sider-logo"/>
        <Menu onClick={this.handleSiderClick} theme="dark" mode="inline" defaultSelectedKeys={['1']}>
          {
            menus.map((it) => {
              return Authorized.check(it.authority, (<Menu.SubMenu key={it.key} title={(<span><Icon type={it.icon}/><FormattedMessage {...it.label}/></span>)}>
                {
                  it.children.map((jt) => {
                    return Authorized.check(jt.authority, (<Menu.Item key={jt.key}>
                      <FormattedMessage {...jt.label}/>
                    </Menu.Item>), null)
                  })
                }
              </Menu.SubMenu>), null)
            })
          }
        </Menu>
      </Sider>
      <Layout>
        <Header style={{
            background: '#fff',
            padding: 0
          }}>
          <Menu onClick={this.handleHeaderClick} mode="horizontal">
            <Menu.Item key='toggle'>
              <Icon className="trigger" type={this.state.collapsed
                  ? 'menu-unfold'
                  : 'menu-fold'}/>
            </Menu.Item>
            <Menu.Item key="search">
              <Authorized authority={ALL} noMatch={null}>
                <HeaderSearch placeholder="站内搜索"/>
              </Authorized>
            </Menu.Item>
            <Menu.Item key="doc">
              <Icon type="question-circle-o"/>
            </Menu.Item>
            <Menu.Item>
              <Authorized authority={ALL} noMatch={null}>
                <NoticeBar/>
              </Authorized>
            </Menu.Item>
            <Menu.Item key="sign-out">
              <Authorized authority={ALL} noMatch={null}>
                <Icon type="logout"/>
              </Authorized>
            </Menu.Item>
          </Menu>
        </Header>
        <Content style={{
            margin: '24px 16px',
            padding: 24,
            background: '#fff',
            minHeight: 360
          }}>
          <Switch>
            {routes.map((it, id) => (<Route key={it.path} exact={true} path={it.path} component={show(it)}/>))}
            <Route component={() => (<Exception type="404"/>)}/>
          </Switch>
        </Content>
        <Footer/>
      </Layout>
    </Layout>)
  }
}

Widget.propTypes = {
  push: PropTypes.func.isRequired,
  intl: intlShape.isRequired,
  title: PropTypes.object.isRequired,
  user: PropTypes.object.isRequired,
  signIn: PropTypes.func.isRequired,
  signOut: PropTypes.func.isRequired
}

export default connect(state => ({title: state.pageTitle, user: state.currentUser}), {push, signIn, signOut})(injectIntl(Widget))
