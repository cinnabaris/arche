import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {
  Form,
  Row,
  Col,
  Input,
  Select,
  message
} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'

import Layout from '../../../../layouts/dashboard'
import {post, get} from '../../../../ajax'
import {Submit, orders, formItemLayout} from '../../../../components/form'
import {ADMIN} from '../../../../auth'

const FormItem = Form.Item
const Option = Select.Option

class Widget extends Component {
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    const {id} = this.props.match.params
    if (id) {
      get(`/admin/links/${id}`).then((rst) => setFieldsValue({label: rst.label, href: rst.href, x: rst.x.toString(), y: rst.y.toString(), loc: rst.loc})).catch(message.error)
    } else {
      setFieldsValue({x: '0', y: '0', loc: 'header'})
    }
  }
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    const {push} = this.props
    const {id} = this.props.match.params
    e.preventDefault();
    this.props.form.validateFields((err, values) => {
      if (!err) {
        post(
          id
          ? `/admin/links/${id}`
          : '/admin/links',
        Object.assign({}, values, {
          x: parseInt(values.x, 10),
          y: parseInt(values.y, 10)
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
          push('/admin/links')
        }).catch(message.error);
      }
    });
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    const {id} = this.props.match.params
    const title = id
      ? {
        id: "buttons.edit",
        values: {
          id: id
        }
      }
      : {
        id: "buttons.new"
      }
    return (<Layout breads={[
        {
          href: '/admin/links',
          label: {
            id: 'nut.admin.links.index.title'
          }
        },
        id
          ? {
            href: `/admin/links/edit/${id}`,
            label: title
          }
          : {
            href: "/admin/links/new",
            label: title
          }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.loc" />}>
              {
                getFieldDecorator('loc')(<Select>
                  {['header', 'footer', 'sidebar'].map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.x" />}>
              {
                getFieldDecorator('x')(<Select>
                  {orders(10).map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.y" />}>
              {
                getFieldDecorator('y')(<Select>
                  {orders(10).map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.label" />} hasFeedback={true}>
              {
                getFieldDecorator('label', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.href" />} hasFeedback={true}>
              {
                getFieldDecorator('href', {
                  rules: [
                    {
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
      </Row>
    </Layout>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired
}

const WidgetF = Form.create()(injectIntl(Widget))

export default connect(state => ({}), {
  push
},)(WidgetF)
