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
import Card from '../../components/layouts/Card'
import {
  client,
  failed
} from '../../utils/request'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($name: String!, $logo: String!){
            updateUserProfile(name: $name, logo: $logo) {
              createdAt
            }
          }`, {
          logo: values.logo,
          name: values.name
        }).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info{
        getUserProfile {
          name,
          logo,
          email
        }
      }`).then((rst) => {
      setFieldsValue(rst.getUserProfile)
    }).catch(failed)
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator
    } = this.props.form
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

export default Form.create()(injectIntl(Widget))