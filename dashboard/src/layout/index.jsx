import React, {
  Component
} from 'react'
import withRouter from 'umi/withRouter'
import PropTypes from 'prop-types'
import {
  connect
} from 'dva'
import {
  addLocaleData,
  IntlProvider
} from 'react-intl'
import {
  LocaleProvider
} from 'antd'

import {
  client,
  failed
} from '../utils/request'
import {
  detect as detectIntl
} from '../utils/locale'
import {
  get as getToken
} from '../utils/token'
import Layout from './Layout'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      intl: {
        locale: 'en-US',
        messages: {}
      }
    }
  }
  componentDidMount() {
    const {
      dispatch
    } = this.props
    const intl = detectIntl();

    var token = getToken()
    if (token) {
      dispatch({
        type: 'currentUser/sign-in',
        token
      })
    }
    client().request(`query locales($lang: String!){
      listLocaleByLang(lang: $lang) {
        code, message
      }
      listUserPolicy {
        role, resource
      }
    }`, {
      lang: intl.locale
    }).then((rst) => {
      addLocaleData(intl.data)
      var messages = rst.listLocaleByLang.reduce((ar, it) => {
        ar[it.code] = it.message
        return ar
      }, {})
      this.setState({
        intl: {
          locale: intl.locale,
          messages
        }
      })
      dispatch({
        type: 'currentUser/refresh',
        authority: rst.listUserPolicy,
      });
    }).catch(failed)
  }
  render() {
    const {
      children
    } = this.props
    const {
      intl
    } = this.state
    return (<IntlProvider locale={intl.locale} messages={intl.messages}>
      <LocaleProvider locale={intl.antd}>
        <Layout>{children}</Layout>
      </LocaleProvider>
    </IntlProvider>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
}

export default withRouter(connect()(Widget))