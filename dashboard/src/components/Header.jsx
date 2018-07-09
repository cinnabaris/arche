import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Link} from "react-router-dom"
import {Helmet} from "react-helmet"
import {intlShape, injectIntl, FormattedMessage} from 'react-intl'

class Widget extends Component {
  render() {
    const {formatMessage} = this.props.intl
    return (<Helmet>
      <title>
        {formatMessage(this.props.title)}&nbsp;|&nbsp;{formatMessage({id: 'site.title'})}
      </title>
    </Helmet>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  title: PropTypes.object.isRequired
}

export default injectIntl(Widget)