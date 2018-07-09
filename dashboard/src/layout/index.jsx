import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Link} from "react-router-dom"
import Exception from 'ant-design-pro/lib/Exception'
import {Route} from "react-router"
import {
  Row,
  Col,
  Card,
  Layout,
  Avatar,
  Menu,
  Icon
} from 'antd'

import Footer from './Footer'
import createLoading from '../loading'
import {routes} from '../router'
import {Switch} from 'react-router-dom'

const {Header, Content} = Layout;

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
    console.log('init application')
  }
  render() {
    const {formatMessage} = this.props.intl
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
            <Card title={(<FormattedMessage id={"site.title"
}/>)} extra={(<a href="/" target="_blank">
                <Icon type="home"/></a>)}>
              <Switch>
                {routes.map((it, id) => (<Route key={it.path} exact={true} path={it.path} component={createLoading(it.component)}/>))}
                <Route component={() => (<Exception type="404"/>)}/>
              </Switch>
            </Card>
            <br/>

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
  title: PropTypes.object.isRequired
}

export default connect(state => ({title: state.pageTitle}), {push})(injectIntl(Widget))
