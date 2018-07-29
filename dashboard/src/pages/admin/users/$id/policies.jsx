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
  Switch,
  DatePicker,
  message
} from 'antd'
import {
  withRouter
} from 'react-router-dom'

import {
  Submit,
  formItemLayout,
  DATE_FORMAT
} from '../../../../components/form'
import Card from '../../../../components/layouts/Card'
import {
  client,
  failed
} from '../../../../utils/request'
import Authorized from '../../../../components/Authorized'
import {
  is_administrator,
  MANAGER
} from '../../../../utils/authorized'


const FormItem = Form.Item
const RangePicker = DatePicker.RangePicker;

class Widget extends Component {
  handleSubmit = (e) => {
    e.preventDefault()
    const {
      formatMessage
    } = this.props.intl
    const {
      match
    } = this.props
    this.props.form.validateFields((err, values) => {
      if (!err) {
        var policies = JSON.stringify(Object.keys(values).reduce((ar, it) => {
          if (values[it]) {
            if (it.startsWith('s-')) {
              ar.push({
                role: it.substring(2),
              })
            } else if (it.startsWith('m-')) {
              ar.push({
                role: MANAGER,
                resource: it.substring(2),
              })
            }
          }
          return ar
        }, []))

        client().request(`mutation form($user: String!, $policies: String!, $nbf: String!, $exp: String!){
            updateUserPolicy(user: $user, policies: $policies, nbf: $nbf, exp: $exp) {
              createdAt
            }
          }`, {
          user: match.params.id,
          nbf: values.dates[0].format(DATE_FORMAT),
          exp: values.dates[1].format(DATE_FORMAT),
          policies
        }).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
        }).catch(failed)
      }
    })
  }

  componentDidMount() {
    const {
      match
    } = this.props
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info($id: String!){
          listManagerByUser(id: $id) {
            role, resource
          }
        }`, {
      id: match.params.id
    }).then((rst) => {
      rst.listManagerByUser.forEach((it) => {
        if (it.resource) {
          if (it.role === MANAGER) {
            // module manager
            let val = {};
            val[`m-${it.resource}`] = true
            setFieldsValue(val)
          }
        } else {
          // system role
          let val = {}
          val[`s-${it.role}`] = true
          setFieldsValue(val)
        }
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

    return (<Authorized check={is_administrator}>
      <Card title={{
        id: "nut.admin.users.policies.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id="attributes.date-range" />} hasFeedback>
            {
              getFieldDecorator('dates', {rules: [{ type: 'array', required: true, message: formatMessage({id:'validations.date-range'}) }]})(<RangePicker />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.admin.users.policies.admin" />} hasFeedback>
            {
              getFieldDecorator('s-admin', { valuePropName: 'checked' })(<Switch/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.admin.users.policies.forum" />} hasFeedback>
            {
              getFieldDecorator('m-forum', { valuePropName: 'checked' })(<Switch/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.admin.users.policies.caring" />} hasFeedback>
            {
              getFieldDecorator('m-caring', { valuePropName: 'checked' })(<Switch/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.admin.users.policies.library" />} hasFeedback>
            {
              getFieldDecorator('m-library', { valuePropName: 'checked' })(<Switch />)
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