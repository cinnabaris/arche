import React, {
  Component
} from 'react'
import router from 'umi/router'
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
  withRouter
} from 'react-router-dom'

import Layout from "../../../components/layouts/UsersSharedLinks"
import {
  Submit,
  formItemLayout
} from '../../../components/form'
import {
  client,
  failed
} from '../../../utils/request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {
      match
    } = this.props
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($token: String!, $password: String!){
            resetUserPassword(token: $token, password: $password) {
              createdAt
            }
          }`, {
          token: match.params.token,
          password: values.password
        }).then((rst) => {
          message.info(formatMessage({
            id: "nut.emails.user.reset-password.success"
          }))
          router.push('/users/sign-in')
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
    if (value && value !== getFieldValue('password')) {
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
    return (<Layout title="nut.users.reset-password.title">
      <Form onSubmit={this.handleSubmit}>
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
      </Form>
    </Layout>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

export default withRouter(Form.create()(injectIntl(Widget)))