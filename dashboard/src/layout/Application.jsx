import React, {Component} from 'react'
import PropTypes from 'prop-types'

class Widget extends Component {
  render() {
    const {children} = this.props
    return (<div>
      <div>application header</div>
      <div>application footer</div>
    </div>)
  }
}

Widget.propTypes = {}

export default Widget
