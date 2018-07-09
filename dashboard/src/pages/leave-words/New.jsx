import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Form, Input, message} from 'antd'

import SharedLinks from '../users/SharedLinks'
import {Submit, formItemLayout} from '../../components/form'
import {client, LEAVE_WORDS_NEW, failed} from '../../request'
import {setPageTitle} from '../../actions'
import Header from '../../components/Header'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client.request(LEAVE_WORDS_NEW, {
          body: values.body,
          mediaType: 'text'
        }).then((rst) => {
          message.info(formatMessage({id: "flashes.success"}))
        }).catch(failed)
      }
    })
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Form onSubmit={this.handleSubmit}>
      <Header title={{
          id: 'nut.leave-words.new.title'
        }}/>
      <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.content" />} extra={<FormattedMessage id = "nut.leave-words.new.body-helper" />} hasFeedback={true}>
        {
          getFieldDecorator('body', {
            rules: [
              {
                required: true,
                message: formatMessage({id: "validations.required"})
              }
            ]
          })(<Input.TextArea rows={8}/>)
        }
      </FormItem>
      <Submit/>
      <br/>
      <SharedLinks/>
    </Form>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  setPageTitle: PropTypes.func.isRequired
}

export default connect(state => ({}), {push, setPageTitle})(Form.create()(injectIntl(Widget)))
