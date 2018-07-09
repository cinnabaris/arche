import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Form, Input, message} from 'antd'
import {withRouter} from 'react-router-dom'

import SharedLinks from './SharedLinks'
import Header from '../../components/Header'
import {Submit, formItemLayout} from '../../components/form'
import {client, USERS_RESET_PASSWORD, failed} from '../../request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {push, match} = this.props
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client.request(USERS_RESET_PASSWORD, {
          token: match.params.token,
          password: values.password
        }).then((rst) => {
          message.info(formatMessage({id: "nut.emails.user.reset-password.success"}))
          push('/users/sign-in')
        }).catch(failed)
      }
    })
  }
  checkPassword = (rule, value, callback) => {
    const {formatMessage} = this.props.intl
    const {getFieldValue} = this.props.form
    if (value && value !== getFieldValue('password')) {
      callback(formatMessage({id: "validations.password-confirmation"}));
    } else {
      callback();
    }
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Form onSubmit={this.handleSubmit}>
      <Header title={{
          id: "nut.users.reset-password.title"
        }}/>
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
      <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.password-confirmation" />} hasFeedback={true}>
        {
          getFieldDecorator('passwordConfirmation', {
            rules: [
              {
                required: true,
                message: formatMessage({id: "validations.required"})
              }, {
                validator: this.checkPassword
              }
            ]
          })(<Input type="password"/>)
        }
      </FormItem>
      <Submit/>
      <br/>
      <SharedLinks/>
    </Form>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired
}

export default withRouter(connect(state => ({}), {push})(Form.create()(injectIntl(Widget))))
