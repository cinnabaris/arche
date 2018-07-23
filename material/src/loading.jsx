import React from 'react'
import Loadable from 'react-loadable'
import RefreshIcon from '@material-ui/icons/Refresh'

const createLoading = (loader) => {
  return Loadable({
    loader: loader,
    loading: () => (<RefreshIcon/>)
  })
}

export default createLoading