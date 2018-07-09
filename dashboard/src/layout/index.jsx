import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import Exception from 'ant-design-pro/lib/Exception'
import {Route} from "react-router"
import {Row, Col, Layout, Menu} from 'antd'

import Footer from './Footer'
import createLoading from '../loading'
import {routes} from '../router'
import {Switch} from 'react-router-dom'
import Authorized, {TOKEN} from '../Authorized'
import {signIn} from '../actions'

const {Header, Content} = Layout

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
      title: null
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
            <Switch>
              {routes.map((it, id) => (<Route key={it.path} exact={true} path={it.path} component={show(it)}/>))}
              <Route component={() => (<Exception type="404"/>)}/>
            </Switch>
            <br/>
          </Col>
        </Row>
      </Content>
      <Footer/>
    </Layout>)
  }
}

Widget.propTypes = {
  push: PropTypes.func.isRequired,
  intl: intlShape.isRequired,
  title: PropTypes.object.isRequired,
  user: PropTypes.object.isRequired,
  signIn: PropTypes.func.isRequired
}

export default connect(state => ({title: state.pageTitle, user: state.currentUser}), {push, signIn})(injectIntl(Widget))
