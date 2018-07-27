import React, {
  Component
} from 'react'
import {
  injectIntl,
  intlShape,
  FormattedMessage
} from 'react-intl'
import router from 'umi/router'
import {
  Form,
  Input,
  message
} from 'antd'
import {
  connect
} from 'dva'

import {
  Submit,
  formItemLayout
} from '../../components/form'
import Layout from "../../components/layouts/UsersSharedLinks"
import {
  client,
  failed
} from '../../utils/request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault()
    const {
      dispatch
    } = this.props
    const {
      formatMessage
    } = this.props.intl
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($email: String!, $password: String!){
            signInUserByEmail(email: $email, password: $password) {
              token
            }
          }`, values).then((rst) => {
          var token = rst.signInUserByEmail.token
          message.info(formatMessage({
            id: "flashes.success"
          }))
          dispatch({
            type: 'currentUser/sign-in',
            token,
          });

          client().request(`query info{
            listUserPolicy {
              role, resource
            }
          }`, {}).then((rst) => {
            dispatch({
              type: 'currentUser/refresh',
              authority: rst.listUserPolicy,
            })
          }).catch(failed)
          router.push('/users/logs')
        }).catch(failed)
      }
    })
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator
    } = this.props.form
    return (<Layout title="nut.users.sign-in.title">
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
    </Layout>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

export default connect()(Form.create()(injectIntl(Widget)))