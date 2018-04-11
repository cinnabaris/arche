import React, {Component} from 'react'
import {Form, Row, Col, Input, message} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'

import Layout from '../../../layouts/dashboard'
import {post} from '../../../ajax'
import {USER, ADMIN} from '../../../auth'
import {Submit, formItemLayout} from '../../../components/form'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    const {setFieldsValue} = this.props.form
    e.preventDefault();
    this.props.form.validateFields((err, values) => {
      if (!err) {
        post('/users/change-password', values).then(() => {
          message.success(formatMessage({id: "flash.success"}))
          setFieldsValue({currentPassword: "", newPassword: "", passwordConfirmation: ""})
        }).catch(message.error);
      }
    });
  }
  checkPassword = (rule, value, callback) => {
    const {formatMessage} = this.props.intl
    const {getFieldValue} = this.props.form
    if (value && value !== getFieldValue('newPassword')) {
      callback(formatMessage({id: "validator.password-confirmation"}));
    } else {
      callback();
    }
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    const title = {
      id: "nut.users.change-password.title"
    }
    return (<Layout breads={[{
          href: "/users/change-password",
          label: title
        }
      ]} title={title} roles={[USER, ADMIN]}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.current-password" />} hasFeedback={true}>
              {
                getFieldDecorator('currentPassword', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input type="password"/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.new-password" />} hasFeedback={true}>
              {
                getFieldDecorator('newPassword', {
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
            <Submit/>
          </Form>
        </Col>
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

const WidgetF = Form.create()(injectIntl(Widget))

export default connect(state => ({}), {},)(WidgetF)
