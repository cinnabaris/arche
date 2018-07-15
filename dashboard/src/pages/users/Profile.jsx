import React, {Component} from 'react'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {Form, Input, message} from 'antd'

import {Submit, formItemLayout} from '../../components/form'
import Card from '../../components/Card'
import {client, USERS_PROFILE_GET, USERS_PROFILE_SET, failed} from '../../request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(USERS_PROFILE_SET, {
          logo: values.logo,
          name: values.name
        }).then((rst) => {
          message.info(formatMessage({id: "flashes.success"}))
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    client().request(USERS_PROFILE_GET).then((rst) => {
      setFieldsValue(rst.getUserProfile)
    }).catch(failed)
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Card title={{
        id: "nut.users.profile.title"
      }}>
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
            })(<Input disabled={true}/>)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.user.logo" />} hasFeedback={true}>
          {
            getFieldDecorator('logo', {
              rules: [
                {
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }
              ]
            })(<Input/>)
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
