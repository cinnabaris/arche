import React, {Component} from 'react'
import PropTypes from 'prop-types'

export class Widget extends Component {
  render() {
    const {body} = this.props
    return (<div className="ql-editor" dangerouslySetInnerHTML={{
        __html: body
      }}/>)
  }
}

Widget.propTypes = {
  body: PropTypes.string.isRequired
}

export default Widget
