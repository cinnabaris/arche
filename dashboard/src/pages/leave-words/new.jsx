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

import Layout from "../../components/layouts/UsersSharedLinks"
import {
  Submit,
  formItemLayout,
  MEDIA_TYPE_TEXT
} from '../../components/form'
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
        client().request(`mutation form($mediaType: String!, $body: String!){
            createLeaveWord(mediaType: $mediaType, body: $body) {
              createdAt
            }
          }`, {
          body: values.body,
          mediaType: MEDIA_TYPE_TEXT
        }).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
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
    return (<Layout title='nut.leave-words.new.title'>
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.content" />} extra={<FormattedMessage id = "nut.leave-words.new.body-helper" />} hasFeedback={true}>
          {
            getFieldDecorator('body', {
              rules: [
              {
                  required: true,
                message: formatMessage({id: "validations.required"})
              }
              ]
            })(<Input.TextArea rows={8}/>)
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