import React, {Component} from 'react'
import {FormattedMessage} from 'react-intl'
import {Link} from 'react-router-dom'
import {Icon, Col} from 'antd'

class Widget extends Component {
  state = {
    items: [
      {
        icon: "user",
        label: "nut.users.sign-in.title",
        href: "/users/sign-in"
      }, {
        icon: "user-add",
        label: "nut.users.sign-up.title",
        href: "/users/sign-up"
      }, {
        icon: "key",
        label: "nut.users.forgot-password.title",
        href: "/users/forgot-password"
      }, {
        icon: "check-circle-o",
        label: "nut.users.confirm.title",
        href: "/users/confirm"
      }, {
        icon: "unlock",
        label: "nut.users.unlock.title",
        href: "/users/unlock"
      }, {
        icon: "message",
        label: "nut.leave-words.new.title",
        href: "/leave-words/new"
      }
    ]
  }
  render() {
    return (<Col md={{
        span: 8,
        offset: 6
      }}>
      {
        this.state.items.map((it) => (<p key={it.href}>
          <Icon type={it.icon}/>&nbsp;
          <Link to={it.href}>
            <FormattedMessage id={it.label}/>
          </Link>
        </p>))
      }
    </Col>)
  }
}

export default Widget;
