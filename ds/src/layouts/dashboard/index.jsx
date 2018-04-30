import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Layout, BackTop} from 'antd'
import {injectIntl, intlShape} from 'react-intl'
import {connect} from 'react-redux'
import Exception from 'ant-design-pro/lib/Exception'
import {push} from 'react-router-redux'

import Footer from '../Footer'
import Sidebar from './Sidebar'
import Breadcrumb from '../Breadcrumb'
import {signOut} from '../../actions'
import {Authorized} from '../../auth'

const {Header, Content, Sider} = Layout

class Widget extends Component {
  render() {
    const {children, breads, site, title, roles} = this.props
    const {formatMessage} = this.props.intl
    document.title = formatMessage(title) + '|' + site.subhead + '|' + site.title
    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" onCollapse={(collapsed, type) => {
          console.log(collapsed, type);
        }}>
        <div className="logo">{site.subhead}</div>
        <Sidebar/>
      </Sider>
      <Layout>
        <Header style={{
            background: '#fff',
            padding: 0
          }}/>
        <Content style={{
            margin: '0 16px'
          }}>
          <Breadcrumb items={breads}/>
          <div style={{
              padding: 24,
              background: '#fff',
              minHeight: 360
            }}>
            <Authorized authority={roles} noMatch={<Exception type = "403" />}>
              {children}
            </Authorized>
          </div>
        </Content>
        <Footer/>
        <BackTop/>
      </Layout>
    </Layout>);
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  push: PropTypes.func.isRequired,
  signOut: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired,
  site: PropTypes.object.isRequired,
  title: PropTypes.object.isRequired,
  breads: PropTypes.array.isRequired,
  roles: PropTypes.array.isRequired,
  intl: intlShape.isRequired
}

const WidgetI = injectIntl(Widget)

export default connect(state => ({user: state.currentUser, site: state.siteInfo}), {push, signOut})(WidgetI)
