import Loadable from 'react-loadable'

import Loading from '../routes/Loading'

export const createLoading = (loader) => {
  return Loadable({loader: loader, loading: Loading})
}
