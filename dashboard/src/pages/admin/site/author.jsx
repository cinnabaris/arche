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
} from '../../../components/form'
import Card from '../../../components/layouts/Card'
import {
  client,
  failed
} from '../../../utils/request'
import Authorized from '../../../components/Authorized'
import {
  is_administrator
} from '../../../utils/authorized'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($name: String!, $email: String!){
            updateSiteAuthor(name: $name, email: $email) {
              createdAt
            }
          }`, values).then((rst) => {
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
        getSiteAuthor{
          name, email
        }
      }`).then((rst) => {
      setFieldsValue(rst.getSiteAuthor)
    }).catch(() => {})
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator
    } = this.props.form
    return (<Authorized check={is_administrator}>
      <Card title={{
        id: "nut.admin.site.author.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.username" />} hasFeedback={true}>
            {
              getFieldDecorator('name', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
            {
              getFieldDecorator('email', {
                rules: [{
                  type: 'email',
                  message: formatMessage({id: "validations.email"})
                }, {
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
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