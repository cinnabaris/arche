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
  message
} from 'antd'
import {
  withRouter
} from 'react-router-dom'

import {
  Submit,
  formItemLayout
} from '../../../../components/form'
import Card from '../../../../components/layouts/Card'
import {
  client,
  failed
} from '../../../../utils/request'
import Authorized from '../../../../components/Authorized'
import {
  is_administrator
} from '../../../../utils/authorized'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault()
    const {
      formatMessage
    } = this.props.intl
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($code: String!, $message: String!){
            updateLocale(code: $code, message: $message) {
              createdAt
            }
          }`, values).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/admin/locales')
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    const {
      match
    } = this.props
    var code = match.params.code
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info($code: String!){
        getLocale(code: $code) {
          code, message
        }
      }`, {
      code
    }).then((rst) => {
      setFieldsValue(rst.getLocale)
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
        id: "nut.admin.locales.index.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.locale.code" />} hasFeedback={true}>
            {
              getFieldDecorator('code')(<Input disabled/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.content" />} hasFeedback={true}>
            {
              getFieldDecorator('message', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input.TextArea rows={8}/>)
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

export default withRouter(Form.create()(injectIntl(Widget)))