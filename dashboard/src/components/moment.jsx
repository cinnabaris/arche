import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {intlShape, injectIntl} from 'react-intl'

import Moment from 'react-moment'

const locale = (l) => {
  switch (l) {
    case 'zh-Hans':
      return 'zh-cn'
    case 'zh-Hant':
      return 'zh-tw'
    default:
      return null
  }
}
class TimestampW extends Component {
  render() {
    const {value, intl} = this.props
    return (<Moment locale={locale(intl.locale)} date={value}/>);
  }
}

TimestampW.propTypes = {
  intl: intlShape.isRequired,
  value: PropTypes.string.isRequired
}

export const Timestamp = injectIntl(TimestampW)

class TimeAgoW extends Component {
  render() {
    const {value, intl} = this.props
    return (<Moment locale={locale(intl.locale)} fromNow={true} ago={true} date={value}/>);
  }
}

TimeAgoW.propTypes = {
  intl: intlShape.isRequired,
  value: PropTypes.string.isRequired
}

export const TimeAgo = injectIntl(TimeAgoW)