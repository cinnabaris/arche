import React, {Component} from 'react'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {Form, Input, message} from 'antd'

import {Submit, formItemLayout} from '../../components/form'
import {client, USERS_CHANGE_PASSWORD, failed} from '../../request'
import Card from '../../components/Card'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(USERS_CHANGE_PASSWORD, {
          currentPassword: values.currentPassword,
          newPassword: values.newPassword
        }).then((rst) => {
          message.info(formatMessage({id: "flashes.success"}))
        }).catch(failed)
      }
    })
  }
  checkPassword = (rule, value, callback) => {
    const {formatMessage} = this.props.intl
    const {getFieldValue} = this.props.form
    if (value && value !== getFieldValue('newPassword')) {
      callback(formatMessage({id: "validations.password-confirmation"}));
    } else {
      callback();
    }
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form

    return (<Card title={{
        id: "nut.users.change-password.title"
      }}>
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.current-password" />} hasFeedback={true}>
          {
            getFieldDecorator('currentPassword', {
              rules: [
                {
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }
              ]
            })(<Input type="password"/>)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.new-password" />} hasFeedback={true}>
          {
            getFieldDecorator('newPassword', {
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
      </Form>
    </Card>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

export default connect(state => ({}), {})(Form.create()(injectIntl(Widget)))
