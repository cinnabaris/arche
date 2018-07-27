import React, {
  Component
} from 'react'
import router from 'umi/router'
import {
  Spin
} from 'antd'

import {
  get as getToken
} from '../utils/token'

class Widget extends Component {
  componentDidMount() {
    router.push(getToken() ? '/users/logs' : '/users/sign-in')
  }
  render() {
    return (<Spin size="large"/>)
  }
}

export default Widget