import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Layout, message} from 'antd'
import {connect} from 'react-redux'
import {FormattedMessage} from 'react-intl'

import {set as setLocale} from '../intl'
import {signIn, refresh} from '../actions'
import {get} from '../ajax'
import {getToken} from '../auth'

const {Footer} = Layout

class Widget extends Component {
  switchLanguage = (l) => {
    setLocale(l)
    window.location.reload()
  }
  componentDidMount() {
    const {signIn, refresh, site, user} = this.props
    if (!user.uid) {
      var tkn = getToken()
      if (tkn) {
        signIn(tkn)
      }
    }
    if (site.languages.length === 0) {
      get('/layout').then((rst) => refresh(rst)).catch(message.error)
    }
  }
  render() {
    const {site} = this.props

    return (<Footer style={{
        textAlign: 'center'
      }}>
      &copy;{site.copyright}
      {
        site.languages.map((l, i) => (<a style={{
            paddingLeft: '8px'
          }} key={i} onClick={(e) => this.switchLanguage(l)}><FormattedMessage id={`languages.${l}`}/></a>))
      }
    </Footer>);
  }
}
Widget.propTypes = {
  refresh: PropTypes.func.isRequired,
  signIn: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired,
  site: PropTypes.object.isRequired
}

export default connect(state => ({user: state.currentUser, site: state.siteInfo}), {signIn, refresh})(Widget)
