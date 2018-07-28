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
  Select,
  message
} from 'antd'
import {
  withRouter
} from 'react-router-dom'

import {
  Submit,
  formItemLayout,
  sort_orders
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
      match
    } = this.props
    const {
      formatMessage
    } = this.props.intl
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($id: String!, $title: String!, $home: String!, $logo: String!, $position: Int!){
            updateFriendLink(id: $id, title: $title, home: $home, logo: $logo, position: $position) {
              createdAt
            }
          }`, Object.assign({}, values, {
          id: match.params.id,
        })).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/admin/friend-links')
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    const {
      match
    } = this.props
    var id = match.params.id
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info($id: String!){
        showFriendLink(id: $id) {
          title, home, logo, position
        }
      }`, {
      id
    }).then((rst) => {
      setFieldsValue(rst.showFriendLink)
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
        id: "nut.admin.friend-links.index.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.title" />} hasFeedback={true}>
            {
              getFieldDecorator('title', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.url" />} hasFeedback={true}>
            {
              getFieldDecorator('home', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.logo" />} hasFeedback={true}>
            {
              getFieldDecorator('logo', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.position" />} hasFeedback={true}>
            {
              getFieldDecorator('position', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {sort_orders().map((it)=>(<Select.Option key={it} value={it}>{it}</Select.Option>))}
              </Select>)
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