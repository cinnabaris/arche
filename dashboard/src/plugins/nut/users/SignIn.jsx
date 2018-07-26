import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  injectIntl,
  intlShape,
  FormattedMessage
} from 'react-intl'
import {
  connect
} from 'react-redux'
import {
  push
} from 'connected-react-router'
import {
  Form,
  Input,
  message
} from 'antd'

import {
  Submit,
  formItemLayout
} from '../../../components/form'
import {
  client,
  failed
} from '../../../request'
import Layout from './Layout'
import {
  signIn
} from '../../../actions'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {
      push,
      signIn
    } = this.props
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
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
          signIn(token)
          push('/users/logs')
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
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  signIn: PropTypes.func.isRequired
}

export default connect(state => ({}), {
  push,
  signIn
})(Form.create()(injectIntl(Widget)))