import React, {
  Component
} from 'react'
import {
  Form,
  Button
} from 'antd'
import {
  FormattedMessage
} from 'react-intl'

const FormItem = Form.Item

export const DATE_FORMAT = "YYYY-MM-DD"

export const formItemLayout = {
  labelCol: {
    xs: {
      span: 24
    },
    sm: {
      span: 6
    }
  },
  wrapperCol: {
    xs: {
      span: 24
    },
    sm: {
      span: 18
    }
  }
}

export const tailFormItemLayout = {
  wrapperCol: {
    xs: {
      span: 24,
      offset: 0
    },
    sm: {
      span: 16,
      offset: 8
    }
  }
}

export class Submit extends Component {
  render() {
    const {
      children
    } = this.props
    return (<FormItem {...tailFormItemLayout}>
      <Button type="primary" htmlType="submit">
        <FormattedMessage id="buttons.submit"/>
      </Button>
      {children}
    </FormItem>);
  }
}