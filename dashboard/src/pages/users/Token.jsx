import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'connected-react-router'
import {message} from 'antd'
import {withRouter} from 'react-router-dom'

import {client, failed} from '../../request'

class Widget extends Component {
  componentDidMount() {
    const {push, action, query, match} = this.props
    const {formatMessage} = this.props.intl
    client().request(query, match.params).then((rst) => {
      message.info(formatMessage({id: `nut.emails.user.${action}.success`}))
      push('/users/sign-in')
    }).catch(failed)
  }
  render() {
    return (<div/>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  action: PropTypes.string.isRequired,
  query: PropTypes.string.isRequired
}

export default withRouter(connect(state => ({}), {push})(injectIntl(Widget)))
