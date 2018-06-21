import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Form, Row, Col, Input, message} from 'antd'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'

import Layout from '../../../layouts/dashboard'
import {post, get} from '../../../ajax'
import {ADMIN} from '../../../auth'
import {Submit, Quill, formItemLayout} from '../../../components/form'

const FormItem = Form.Item

class Widget extends Component {
  state = {
    body: ''
  }
  componentDidMount() {
    const {setFieldsValue} = this.props.form
    const {id} = this.props.match.params
    if (id) {
      get(`/donate/projects/${id}`).then((rst) => {
        setFieldsValue({title: rst.title, methods: rst.methods})
        this.setState({body: rst.body})
      }).catch(message.error)
    }
  }
  handleChange = (value) => {
    this.setState({body: value})
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
          ? `/donate/projects/${id}`
          : '/donate/projects',
        Object.assign({}, values, {
          type: 'html',
          body: this.state.body
        })).then(() => {
          message.success(formatMessage({id: "flash.success"}))
          push('/donate/projects')
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
          href: '/donate/projects',
          label: {
            id: 'donate.projects.index.title'
          }
        },
        id
          ? {
            href: `/donate/projects/edit/${id}`,
            label: title
          }
          : {
            href: "/donate/projects/new",
            label: title
          }
      ]} title={title} roles={[ADMIN]}>
      <Row>
        <Col md={{
            span: 18
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
            <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.body" />}>
              <Quill value={this.state.body} onChange={this.handleChange}/>
            </FormItem>
            <FormItem {...formItemLayout} label={<FormattedMessage id = "donate.attributes.project.methods" />} extra={<FormattedMessage id = "nut.leave-words.new.body-helper" />} hasFeedback={true}>
              {
                getFieldDecorator('methods', {
                  rules: [
                    {
                      required: true,
                      message: formatMessage({id: "validator.required"})
                    }
                  ]
                })(<Input.TextArea rows={8}/>)
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
