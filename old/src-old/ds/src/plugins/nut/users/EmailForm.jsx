import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Form, Row, Col, Input, message} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'

import Layout from '../../../layouts/application'
import {post} from '../../../ajax'
import {Submit, formItemLayout} from '../../../components/form'
import SharedLinks from './SharedLinks'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {push, action} = this.props
    const {formatMessage} = this.props.intl
    e.preventDefault();
    this.props.form.validateFields((err, values) => {
      if (!err) {
        post(`/users/${action}`, Object.assign({}, values, {
          next: window.location.href + '/../sign-in'
        })).then(() => {
          message.info(formatMessage({id: `nut.users.${action}.success`}))
          push('/users/sign-in')
        }).catch(message.error);
      }
    });
  }
  render() {
    const {action} = this.props
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    const title = {
      id: `nut.users.${action}.title`
    }
    return (<Layout breads={[{
          href: `/users/${action}`,
          label: title
        }
      ]} title={title}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
              {
                getFieldDecorator('email', {
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
            <Submit/>
          </Form>
        </Col>
        <SharedLinks/>
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  action: PropTypes.string.isRequired,
  push: PropTypes.func.isRequired
}

const WidgetF = Form.create()(injectIntl(Widget))

export default connect(state => ({}), {
  push
},)(WidgetF)
