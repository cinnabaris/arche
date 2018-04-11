import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {connect} from 'react-redux'
import Moment from 'react-moment'
import 'moment-timezone'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import 'moment/locale/en-gb'

export class Widget extends Component {
  render() {
    const {value, site} = this.props
    var locale;
    switch (site.locale) {
      case "zh-Hans":
        locale = 'zh-cn'
        break;
      case "zh-Hant":
        locale = 'zh-tw'
        break
      default:
        locale = 'en-gb'
    }
    return (<Moment locale={locale} fromNow={true}>{value}</Moment>)
  }
}

Widget.propTypes = {
  value: PropTypes.string.isRequired,
  site: PropTypes.object.isRequired
}

export default connect(state => ({site: state.siteInfo}), {})(Widget)
