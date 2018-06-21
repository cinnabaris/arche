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
import {Submit, Quill, orders, formItemLayout} from '../../../../components/form'

const FormItem = Form.Item
const Option = Select.Option

class Widget extends Component {
  state = {
    summary: ''
  }
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    const {id} = this.props.match.params
    if (id) {
      get(`/admin/cards/${id}`).then((rst) => {
        setFieldsValue({
          title: rst.title,
          action: rst.action,
          logo: rst.logo,
          href: rst.href,
          sort: rst.sort.toString(),
          loc: rst.loc
        })
        this.setState({summary: rst.summary})
      }).catch(message.error)
    } else {
      setFieldsValue({sort: '0', loc: 'album-main'})
    }
  }
  handleChange = (value) => {
    this.setState({summary: value})
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
          ? `/admin/cards/${id}`
          : '/admin/cards',
        Object.assign({}, values, {
          sort: parseInt(values.sort, 10),
          type: 'html',
          summary: this.state.summary
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
          push('/admin/cards')
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
          href: '/admin/cards',
          label: {
            id: 'nut.admin.cards.index.title'
          }
        },
        id
          ? {
            href: `/admin/cards/edit/${id}`,
            label: title
          }
          : {
            href: "/admin/cards/new",
            label: title
          }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 18
          }}>
          <Form onSubmit={this.handleSubmit}>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.loc" />}>
              {
                getFieldDecorator('loc')(<Select>
                  {['album-header', 'album-main', 'carousel-top', 'carousel-middle', 'carousel-bottom'].map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.sort-order" />}>
              {
                getFieldDecorator('sort')(<Select>
                  {orders(10).map((p) => (<Option key={p} value={p}>{p}</Option>))}
                </Select>)
              }
            </FormItem>
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
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.summary" />}>
              <Quill value={this.state.summary} onChange={this.handleChange}/>
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.action" />} hasFeedback={true}>
              {
                getFieldDecorator('action', {
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
