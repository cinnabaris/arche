import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Col,
  Card,
  Icon,
  Menu
} from 'antd'
import {
  FormattedMessage
} from 'react-intl'
import router from 'umi/router'

import Head from '../Head'

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
  render() {
    const {
      children,
      title
    } = this.props
    return (<Col xs={{span: 22, offset: 1}} lg={{span: 14, offset: 2}}>
      <Card title={(<FormattedMessage id={title}/>)} extra={(<Icon onClick={()=>window.open("/", "_blank")} type="home"/>)}>
        {children}
        <Menu
          onClick={(e)=>router.push(e.key)}
          mode="inline"
        >
          {this.state.items.map((it)=>(<Menu.Item key={it.to}>
            <Icon type={it.icon} />
            <FormattedMessage id={it.label}/>
          </Menu.Item>))}
        </Menu>
        <Head title={{id: title}}/>
      </Card>
    </Col>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  title: PropTypes.string.isRequired
}

export default Widget