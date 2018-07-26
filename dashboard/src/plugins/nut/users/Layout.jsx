import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Row,
  Col,
  Card,
  Icon,
  Menu
} from 'antd'
import {
  FormattedMessage
} from 'react-intl'
import {
  connect
} from 'react-redux'
import {
  push
} from 'connected-react-router'
import {
  withRouter
} from 'react-router-dom'

import Header from '../../../components/Header'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [{
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
      }]
    }
  }
  goHome = () => {
    window.open("/", "_blank")
  }
  handleClick = (e) => {
    const {
      push
    } = this.props
    push(e.key)
    // console.log(history)
    // console.log(e.key)
  }
  render() {
    const {
      children,
      title
    } = this.props
    return (<Row>
      <Col xs={{
          span: 22,
          offset: 1
      }} lg={{
          span: 12,
          offset: 6
      }}>
        <Card title={(<FormattedMessage id={title}/>)} extra={(<Icon onClick={this.goHome} type="home"/>)}>
          {children}
          <Menu
            onClick={this.handleClick}
            mode="inline"
          >
            {this.state.items.map((it)=>(<Menu.Item key={it.to}>
              <Icon type={it.icon} />
              <FormattedMessage id={it.label}/>
            </Menu.Item>))}
          </Menu>
          <Header title={{id: title}}/>
          </Card>
      </Col>
    </Row>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  push: PropTypes.func.isRequired,
  title: PropTypes.string.isRequired
}

export default withRouter(connect(state => ({}), {
  push
})(Widget))