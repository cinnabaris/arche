import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {
  Form,
  Row,
  Col,
  Input,
  Select,
  Button,
  Popconfirm,
  message
} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'

import Layout from '../../../../layouts/dashboard'
import {post, get, patch} from '../../../../ajax'
import {ADMIN} from '../../../../auth'
import {Submit, formItemLayout} from '../../../../components/form'

const FormItem = Form.Item
const Option = Select.Option

class Widget extends Component {
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    get('/admin/site/smtp').then((rst) => setFieldsValue(Object.assign({}, rst, {port: rst.port.toString()}))).catch(message.error)
  }
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    e.preventDefault();
    this.props.form.validateFields((err, values) => {
      if (!err) {
        post('/admin/site/smtp', Object.assign({}, values, {
          port: parseInt(values.port, 10)
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
        }).catch(message.error);
      }
    });
  }
  checkPassword = (rule, value, callback) => {
    const {formatMessage} = this.props.intl
    const {getFieldValue} = this.props.form
    if (value && value !== getFieldValue('password')) {
      callback(formatMessage({id: "validator.password-confirmation"}));
    } else {
      callback();
    }
  }
  handleTest = () => {
    const {formatMessage} = this.props.intl
    this.props.form.validateFields((err, values) => {
      if (!err) {
        patch('/admin/site/smtp', Object.assign({}, values, {
          port: parseInt(values.port, 10)
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
        }).catch(message.error);
      }
    });
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    const title = {
      id: "nut.admin.site.smtp.title"
    }
    return (<Layout breads={[{
          href: "/admin/site/smtp",
          label: title
        }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.host" />} hasFeedback={true}>
              {
                getFieldDecorator('host', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.port" />}>
              {
                getFieldDecorator('port')(<Select>
                  {["25", "465", "587"].map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.user" />} hasFeedback={true}>
              {
                getFieldDecorator('username', {
                  rules: [
                    {
                      type: 'email',
                      message: formatMessage({id: "validator.email"})
                    }, {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.password" />} hasFeedback={true}>
              {
                getFieldDecorator('password', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
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
                      message: formatMessage({id: "validator.required"})
                    }, {
                      validator: this.checkPassword
                    }
                  ]
                })(<Input type="password"/>)
              }
            </FormItem>
            <Submit>
              &nbsp; &nbsp;
              <Popconfirm title={<FormattedMessage id = "helpers.are-you-sure" />} onConfirm={this.handleTest}>
                <Button type="danger">
                  <FormattedMessage id="buttons.test"/>
                </Button>
              </Popconfirm>
            </Submit>
          </Form>
        </Col>
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired
}

const WidgetF = Form.create()(injectIntl(Widget))

export default connect(state => ({user: state.currentUser}), {
  push
},)(WidgetF)
