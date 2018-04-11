import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Layout, BackTop} from 'antd';
import {connect} from 'react-redux'
import {injectIntl, intlShape} from 'react-intl'

import Footer from '../Footer'
import Header from './Header'
import Breadcrumb from '../Breadcrumb'

const {Content} = Layout;

class Widget extends Component {
  render() {
    const {children, breads, site, title} = this.props
    const {formatMessage} = this.props.intl
    document.title = formatMessage(title) + '|' + site.subhead + '|' + site.title
    return (<Layout className="layout">
      <Header/>
      <Content style={{
          padding: '0 50px'
        }}>
        <Breadcrumb items={breads}/>
        <div style={{
            background: '#fff',
            padding: 24,
            minHeight: 280
          }}>{children}</div>
      </Content>
      <Footer/>
      <BackTop/>
    </Layout>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  title: PropTypes.object.isRequired,
  site: PropTypes.object.isRequired,
  intl: intlShape.isRequired,
  breads: PropTypes.array.isRequired
}

const WidgetI = injectIntl(Widget)
export default connect(state => ({site: state.siteInfo}))(WidgetI)
