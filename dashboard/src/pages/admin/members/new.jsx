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
  DatePicker,
  Select,
  message
} from 'antd'

import {
  Submit,
  DATE_FORMAT,
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
        client().request(`mutation form($nickName: String!, $realName: String!, $gender: String!, $birthday: String!, $phone: String, $email: String, $address: String, $line: String, $wechat: String, $skype: String, $weibo: String, $facebook: String){
            createMember(nickName: $nickName, realName: $realName, gender: $gender, birthday: $birthday, phone: $phone, email: $email, address: $address, line: $line, wechat: $wechat, skype: $skype, weibo: $weibo, facebook: $facebook) {
              createdAt
            }
          }`, Object.assign({}, values, {
          birthday: values.birthday.format(DATE_FORMAT)
        })).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/admin/members')
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
    return (<Authorized check={is_administrator}>
      <Card title={{
        id: "nut.admin.members.index.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.nick-name" />} hasFeedback={true}>
            {
              getFieldDecorator('nickName', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.real-name" />} hasFeedback={true}>
            {
              getFieldDecorator('realName', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.gender" />} hasFeedback={true}>
            {
              getFieldDecorator('gender', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {['m', 'f'].map((it)=>(<Select.Option key={it} value={it}><FormattedMessage id={`attributes.gender-${it}`}/></Select.Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.birthday" />} hasFeedback={true}>
            {
              getFieldDecorator('birthday', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.date"})
                }]
              })(<DatePicker/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.phone" />} hasFeedback={true}>
            {
              getFieldDecorator('phone')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.address" />} hasFeedback={true}>
            {
              getFieldDecorator('address')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
            {
              getFieldDecorator('email', {
                rules: [{
                  type: 'email',
                  message: formatMessage({id: "validations.email"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.line" />} hasFeedback={true}>
            {
              getFieldDecorator('line')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.wechat" />} hasFeedback={true}>
            {
              getFieldDecorator('wechat')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.skype" />} hasFeedback={true}>
            {
              getFieldDecorator('skype')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.weibo" />} hasFeedback={true}>
            {
              getFieldDecorator('weibo')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.facebook" />} hasFeedback={true}>
            {
              getFieldDecorator('facebook')(<Input/>)
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