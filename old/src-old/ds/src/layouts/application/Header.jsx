import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Layout, Menu} from 'antd';
import {connect} from 'react-redux'
import {Link} from 'react-router-dom'
import {FormattedMessage} from 'react-intl'

const {Header} = Layout;

class Widget extends Component {
  render() {
    // const width = document.body.offsetWidth
    const {site, user} = this.props
    return (<Header>
      <div className="logo"/>
      <Menu theme="dark" mode="horizontal" defaultSelectedKeys={[]} style={{
          lineHeight: '64px'
        }}>
        <Menu.Item key="home">
          <Link to="/">{site.subhead}</Link>
        </Menu.Item>
        <Menu.Item key="personal">
          <Link to={user.uid
              ? '/users/logs'
              : '/users/sign-in'}>
            <FormattedMessage id={user.uid
                ? 'nut.dashboard.title'
                : 'nut.users.sign-in.title'}/>
          </Link>
        </Menu.Item>
      </Menu>
    </Header>)
  }
}

Widget.propTypes = {
  user: PropTypes.object.isRequired,
  site: PropTypes.object.isRequired
}

export default connect(state => ({user: state.currentUser, site: state.siteInfo}))(Widget)
