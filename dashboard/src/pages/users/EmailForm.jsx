import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Form, Input, message} from 'antd'

import Layout from './Layout'
import {Submit, formItemLayout} from '../../components/form'
import {client, failed} from '../../request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {push, action, query} = this.props
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(query, values).then((rst) => {
          message.info(formatMessage({id: `nut.users.${action}.success`}))
          push('/users/sign-in')
        }).catch(failed)
      }
    })
  }
  render() {
    const {action} = this.props
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Layout title={`nut.users.${action}.title`}>
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
        <Submit/>
      </Form>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  action: PropTypes.string.isRequired,
  query: PropTypes.string.isRequired
}

export default connect(state => ({}), {push})(Form.create()(injectIntl(Widget)))
