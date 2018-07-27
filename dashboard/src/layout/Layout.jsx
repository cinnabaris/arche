import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  injectIntl,
  intlShape,
  FormattedMessage
} from 'react-intl'
import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import RenderAuthorized from 'ant-design-pro/lib/Authorized'
import {
  Icon,
  Layout,
  Menu,
  message,
  Modal,
  Row
} from 'antd'
import router from 'umi/router'
import {
  connect
} from 'dva'

import {
  client,
  failed
} from '../utils/request'
import {
  is_sign_in
} from '../utils/authorized'
import NoticeBar from './NoticeBar'
import Footer from './Footer'
import menus from './menus'

const {
  Header,
  Sider,
  Content
} = Layout

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {}
  }
  handleHeaderClick = (e) => {
    const {
      dispatch
    } = this.props
    const {
      formatMessage
    } = this.props.intl
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
          title: formatMessage({
            id: "header.sign-out.confirm"
          }),
          onOk() {
            client().request(`mutation form{
              signOutUser {
                createdAt
              }
            }`, {}).then((rst) => {
              router.push('/users/sign-in')
              message.info(formatMessage({
                id: "flashes.success"
              }))
              dispatch({
                type: 'currentUser/sign-out'
              });
            }).catch(failed)
          }
        })
        return
      default:
        console.log(e.key)
    }
  }
  render() {
    const {
      children,
      currentUser
    } = this.props
    const Authorized = RenderAuthorized(currentUser.authority)
    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" trigger={null} collapsible="collapsible" collapsed={this.state.collapsed}>
        <div className="sider-logo"/>
        <Menu onClick={(e)=>router.push(e.key)} theme="dark" mode="inline" defaultSelectedKeys={['1']}>
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
              <Authorized authority={is_sign_in} noMatch={null}>
                <HeaderSearch placeholder="站内搜索"/>
              </Authorized>
            </Menu.Item>
            <Menu.Item key="doc">
              <Icon type="question-circle-o"/>
            </Menu.Item>
            <Menu.Item>
              <Authorized authority={is_sign_in} noMatch={null}>
                <NoticeBar/>
              </Authorized>
            </Menu.Item>
            <Menu.Item key="sign-out">
              <Authorized authority={is_sign_in} noMatch={null}>
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
          <Row>{children}</Row>
        </Content>
        <Footer/>
      </Layout>
    </Layout>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  intl: intlShape.isRequired,
  currentUser: PropTypes.object.isRequired,
}

export default connect(({
  currentUser
}) => ({
  currentUser,
}))(injectIntl(Widget))