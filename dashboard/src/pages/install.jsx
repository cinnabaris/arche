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

import Layout from "../components/layouts/UsersSharedLinks"
import {
  Submit,
  formItemLayout
} from '../components/form'
import {
  client,
  failed
} from '../utils/request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($name: String!, $email: String!, $password: String!){
            install(name: $name, email: $email, password: $password) {
              createdAt
            }
          }`, values).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
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
    return (<Layout title="nut.install.title">
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.username" />} hasFeedback={true}>
          {
            getFieldDecorator('name', {
                rules: [
              {
                required: true,
                message: formatMessage({id: "validations.required"})
              }
                ]
            })(<Input/>)
          }
        </FormItem>
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

export default Form.create()(injectIntl(Widget))