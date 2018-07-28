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
        client().request(`mutation form($title: String!, $subhead: String!, $keywords: String!, $description: String!, $copyright: String!){
            updateSiteInfo(title: $title, subhead: $subhead, keywords: $keywords, description: $description, copyright: $copyright) {
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
        getSiteInfo{
          title, subhead, keywords, description, copyright
        }
      }`).then((rst) => {
      setFieldsValue(rst.getSiteInfo)
    }).catch(failed)
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
        id: "nut.users.profile.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.site.title" />} hasFeedback>
            {
              getFieldDecorator('title', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.site.subhead" />} hasFeedback>
            {
              getFieldDecorator('subhead', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.site.keywords" />} hasFeedback>
            {
              getFieldDecorator('keywords', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.site.description" />} hasFeedback>
            {
              getFieldDecorator('description', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input.TextArea rows={8}/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.site.copyright" />} hasFeedback>
            {
              getFieldDecorator('copyright', {
                rules: [{
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