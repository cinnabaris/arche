import React, {Component} from 'react'
import PropTypes from 'prop-types'

export class Widget extends Component {
  render() {
    const {body, length} = this.props
    var tmp = document.createElement("div")
    tmp.innerHTML = body
    var txt = tmp.textContent || tmp.innerText || ""
    return <code>{txt.substring(0, length)}</code>
  }
}

Widget.propTypes = {
  body: PropTypes.string.isRequired,
  length: PropTypes.number.isRequired
}

export default Widget
