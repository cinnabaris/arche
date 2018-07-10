import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {
  List,
  Row,
  Col,
  Card,
  Avatar,
  Icon
} from 'antd'
import {Link} from "react-router-dom"
import {FormattedMessage} from 'react-intl'

import Header from '../../components/Header'

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
  goHome = () => {
    window.open("/", "_blank")
  }
  render() {
    const {children, title} = this.props
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
          <List bordered={true} size="small" itemLayout="horizontal" dataSource={this.state.items} renderItem={it => (<List.Item><List.Item.Meta avatar={(<Avatar icon={it.icon}/>)} description={(<Link to={it.to}><FormattedMessage id={it.label}/></Link>)}/></List.Item>)}/>
          <Header title={{
              id: title
            }}/>
        </Card>
      </Col>
    </Row>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  title: PropTypes.string.isRequired
}

export default Widget
