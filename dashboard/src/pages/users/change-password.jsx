import React, {
  Component
} from 'react'
import {
  injectIntl,
  intlShape,
  FormattedMessage
} from 'react-intl'
import {
  Form,
  Input,
  message
} from 'antd'

import {
  Submit,
  formItemLayout
} from '../../components/form'
import {
  client,
  failed
} from '../../utils/request'
import Card from '../../components/layouts/Card'
import Authorized from '../../components/Authorized'
import {
  is_sign_in
} from '../../utils/authorized'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault()
    const {
      formatMessage
    } = this.props.intl
    const {
      setFieldsValue
    } = this.props.form
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($currentPassword: String!, $newPassword: String!){
            changeUserPassword(currentPassword: $currentPassword, newPassword: $newPassword) {
              createdAt
            }
          }`, {
          currentPassword: values.currentPassword,
          newPassword: values.newPassword
        }).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          setFieldsValue({
            currentPassword: '',
            newPassword: '',
            passwordConfirmation: ''
          })
        }).catch(failed)
      }
    })
  }
  checkPassword = (rule, value, callback) => {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldValue
    } = this.props.form
    if (value && value !== getFieldValue('newPassword')) {
      callback(formatMessage({
        id: "validations.password-confirmation"
      }));
    } else {
      callback();
    }
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator
    } = this.props.form

    return (<Authorized check={is_sign_in}>
      <Card title={{
        id: "nut.users.change-password.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.current-password" />} hasFeedback={true}>
            {
              getFieldDecorator('currentPassword', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input type="password"/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.new-password" />} hasFeedback={true}>
            {
              getFieldDecorator('newPassword', {
                rules: [{
                  required: true,
                  max: 30,
                  min: 6,
                  message: formatMessage({id: "validations.password"})
                }]
              })(<Input type="password"/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.password-confirmation" />} hasFeedback={true}>
            {
              getFieldDecorator('passwordConfirmation', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }, {
                  validator: this.checkPassword
                }]
              })(<Input type="password"/>)
            }
          </FormItem>
          <Submit/>
        </Form>
      </Card>
    </Authorized>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

export default Form.create()(injectIntl(Widget))