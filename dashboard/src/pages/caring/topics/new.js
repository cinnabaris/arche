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
import lodash from 'lodash'
import moment from 'moment'

import {
  Submit,
  formItemLayout,
  MEDIA_TYPE_HTML,
  Quill
} from '../../../components/form'
import Card from '../../../components/layouts/Card'
import {
  client,
  failed
} from '../../../utils/request'
import Authorized from '../../../components/Authorized'
import {
  is_caring_manager
} from '../../../utils/authorized'
import {
  caring_topic_statuses,
  caring_topic_tags
} from '../../../utils/config'

const FormItem = Form.Item

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      members: [],
      reason: ''
    }
  }
  handleReasonChange = (v) => {
    this.setState({
      reason: v
    })
  }
  onMemberChange = (id) => {
    const {
      setFieldsValue
    } = this.props.form
    var it = this.state.members.find((it) => it.id === id)
    setFieldsValue({
      name: it.realName,
      address: it.address,
      phone: it.phone,
      gender: it.gender,
      email: it.email,
      age: moment().diff(it.birthday, 'years')
    })
  }
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($memberId: String!, $tag: String!, $name: String!, $gender: String!, $age: Int!, $phone: String, $email: String, $address: String, $reason: String!, $mediaType: String!, $status: String!){
            createCaringTopic(memberId: $memberId, tag: $tag, name: $name, gender: $gender, age: $age, phone: $phone, email: $email, address: $address, reason: $reason, mediaType: $mediaType, status: $status) {
              createdAt
            }
          }`, Object.assign(values, {
          reason: this.state.reason,
          mediaType: MEDIA_TYPE_HTML
        })).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
          router.push('/caring/topics')
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    client().request(`query list{
        listMember {
          id, nickName, realName, birthday, gender, phone, email, address
        }
      }`, {}).then((rst) => {
      this.setState({
        members: rst.listMember
      })
    }).catch(failed)
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator
    } = this.props.form
    return (<Authorized check={is_caring_manager}>
      <Card title={{
        id: "caring.topics.index.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "caring.attributes.topic.tag" />} hasFeedback={true}>
            {
              getFieldDecorator('tag', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {caring_topic_tags.map((it)=>(<Select.Option key={it} value={it}><FormattedMessage id={`caring.attributes.topic.tag-${it}`}/></Select.Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.member" />} hasFeedback={true}>
            {
              getFieldDecorator('memberId', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select onChange={this.onMemberChange}>
                {this.state.members.map((it)=><Select.Option key={it.id} value={it.id}>{it.nickName}[{it.realName}](<FormattedMessage id={`attributes.gender-${it.gender}`}/>)</Select.Option>)}
              </Select>)
            }
          </FormItem>
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
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.age" />} hasFeedback={true}>
            {
              getFieldDecorator('age', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {lodash.range(1, 120).map((it)=>(<Select.Option key={it} value={it}>{it}</Select.Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.phone" />} hasFeedback={true}>
            {
              getFieldDecorator('phone')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
            {
              getFieldDecorator('email')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.member.address" />} hasFeedback={true}>
            {
              getFieldDecorator('address')(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "caring.attributes.topic.reason" />}>
            <Quill value={this.state.reason} onChange={this.handleReasonChange} />
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.status" />} hasFeedback={true}>
            {
              getFieldDecorator('status', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Select>
                {caring_topic_statuses.map((it)=>(<Select.Option key={it} value={it}><FormattedMessage id={`caring.attributes.topic.status-${it}`}/></Select.Option>))}
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