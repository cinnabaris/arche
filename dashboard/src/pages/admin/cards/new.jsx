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
  Select,
  Form,
  Input,
  message
} from 'antd'

import {
  Submit,
  formItemLayout,
  sort_orders,
  Quill,
  MEDIA_TYPE_HTML
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
import {
  cards_loc_options
} from '../../../utils/config'

const FormItem = Form.Item

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      body: ''
    }
  }
  handleBodyChange = (v) => {
    this.setState({
      body: v
    })
  }
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($title: String!, $body: String!, $mediaType: String!, $action: String!, $href: String!, $logo: String!, $loc: String!, $position: Int!){
            createCard(title: $title, body: $body, mediaType: $mediaType, action: $action, href: $href, logo: $logo, loc: $loc, position: $position) {
              createdAt
            }
          }`, Object.assign({}, values, {
          mediaType: MEDIA_TYPE_HTML,
          body: this.state.body
        })).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/admin/cards')
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
        id: "nut.admin.cards.index.title"
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
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.content" />}>
            <Quill value={this.state.body} onChange={this.handleBodyChange} />
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.url" />} hasFeedback={true}>
            {
              getFieldDecorator('href', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.card.action" />} hasFeedback={true}>
            {
              getFieldDecorator('action', {
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
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.loc" />} hasFeedback={true}>
            {
              getFieldDecorator('loc', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {cards_loc_options.map((it)=>(<Select.Option key={it} value={it}>{it}</Select.Option>))}
              </Select>)
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

export default Form.create()(injectIntl(Widget))