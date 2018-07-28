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
  Select,
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
        client().request(`mutation form($host: String!, $port: Int!, $user: String!, $password: String!){
            updateSiteSmtp(host: $host, port: $port, user: $user, password: $password) {
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
  componentDidMount() {
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info{
        getSiteSmtp{
          host, port, user, password
        }
      }`).then((rst) => {
      setFieldsValue(Object.assign(rst.getSiteSmtp, {
        passwordConfirmation: ''
      }))
    }).catch(() => {
      setFieldsValue({
        port: 25
      })
    })
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
        id: "nut.admin.site.smtp.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.host" />} hasFeedback>
            {
              getFieldDecorator('host', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.port" />} hasFeedback>
            {
              getFieldDecorator('port', {
                rules: []
              })(<Select>
                {[25, 465, 587].map((it)=>(<Select.Option key={it} value={it}>{it}</Select.Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.user" />} hasFeedback={true}>
            {
              getFieldDecorator('user', {
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