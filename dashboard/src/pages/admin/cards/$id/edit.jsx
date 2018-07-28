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
  cards_loc_options
} from '../../../../utils/config'
import {
  Submit,
  formItemLayout,
  sort_orders,
  Quill,
  MEDIA_TYPE_HTML
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
    e.preventDefault()
    const {
      match
    } = this.props
    const {
      formatMessage
    } = this.props.intl
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($id: String!, $title: String!, $body: String!, $mediaType: String!, $action: String!, $href: String!, $logo: String!, $loc: String!, $position: Int!){
            updateCard(id: $id, title: $title, body: $body, mediaType: $mediaType, action: $action, href: $href, logo: $logo, loc: $loc, position: $position){
              createdAt
            }
          }`, Object.assign({}, values, {
          id: match.params.id,
          mediaType: MEDIA_TYPE_HTML,
          body: this.state.body,
        })).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/admin/cards')
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
        showCard(id: $id) {
          title, body, mediaType, action, href, logo, loc, position
        }
      }`, {
      id
    }).then((rst) => {
      this.setState({
        body: rst.showCard.body
      })
      let v = Object.assign({}, rst.showCard)
      delete v.body
      setFieldsValue(v)
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
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.url" />}>
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

export default withRouter(Form.create()(injectIntl(Widget)))