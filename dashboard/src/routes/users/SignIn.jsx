import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Form, Input, message} from 'antd'

import Application from '../../layouts/Application'
import {Submit, formItemLayout} from '../../components/form'
import {client, USERS_SIGN_IN, failed} from '../../request'
import {signIn, TOKEN} from '../../actions'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {push, signIn} = this.props
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client.request(USERS_SIGN_IN, values).then((rst) => {
          message.info(formatMessage({id: "flashes.success"}))
          localStorage.setItem(TOKEN, rst.token)
          signIn(rst.token)
          push('/users/logs')
        }).catch(failed)
      }
    })
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Application title="nut.users.sign-in.title" submit={this.handleSubmit}>
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
          {
            getFieldDecorator('email', {
              rules: [
                {
                  type: 'email',
                  message: formatMessage({id: "validations.email"})
                }, {
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }
              ]
            })(<Input/>)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.password" />} hasFeedback={true}>
          {
            getFieldDecorator('password', {
              rules: [
                {
                  required: true,
                  max: 30,
                  min: 6,
                  message: formatMessage({id: "validations.password"})
                }
              ]
            })(<Input type="password"/>)
          }
        </FormItem>
        <Submit/>
      </Form>
    </Application>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired
}

export default connect(state => ({}), {push, signIn})(Form.create()(injectIntl(Widget)))
