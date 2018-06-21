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
import {ADMIN} from '../../../../auth'
import {Submit, orders, formItemLayout} from '../../../../components/form'

const FormItem = Form.Item
const Option = Select.Option

class Widget extends Component {
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    const {id} = this.props.match.params
    if (id) {
      get(`/admin/friend-links/${id}`).then((rst) => setFieldsValue({title: rst.title, home: rst.home, logo: rst.logo, sort: rst.sort.toString()})).catch(message.error)
    } else {
      setFieldsValue({sort: '0'})
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
          ? `/admin/friend-links/${id}`
          : '/admin/friend-links',
        Object.assign({}, values, {
          sort: parseInt(values.sort, 10)
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
          push('/admin/friend-links')
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
          href: '/admin/friend-links',
          label: {
            id: 'nut.admin.friend-links.index.title'
          }
        },
        id
          ? {
            href: `/admin/friend-links/edit/${id}`,
            label: title
          }
          : {
            href: "/admin/friend-links/new",
            label: title
          }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 12,
            offset: 2
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.title" />} hasFeedback={true}>
              {
                getFieldDecorator('title', {
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
                getFieldDecorator('home', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.logo" />} hasFeedback={true}>
              {
                getFieldDecorator('logo', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input/>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.sort-order" />}>
              {
                getFieldDecorator('sort')(<Select>
                  {orders(10).map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
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
