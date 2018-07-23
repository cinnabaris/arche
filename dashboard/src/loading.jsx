import React from 'react'
import Loadable from 'react-loadable'
import {
  Spin
} from 'antd'


const createLoading = (loader) => {
  return Loadable({
    loader: loader,
    loading: () => (<Spin size="large"/>)
  })
}

export default createLoading