import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Helmet
} from "react-helmet"
import {
  intlShape,
  injectIntl
} from 'react-intl'

class Widget extends Component {
  render() {
    const {
      formatMessage
    } = this.props.intl
    return (<Helmet>
      <title>
        {formatMessage(this.props.title)}|{formatMessage({id: 'site.subhead'})}|{formatMessage({id: 'site.title'})}
      </title>
    </Helmet>);
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  title: PropTypes.object.isRequired
}

export default injectIntl(Widget)