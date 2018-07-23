import React, {
  Component
} from 'react'

import {
  FormattedMessage
} from 'react-intl';

class Widget extends Component {
  render() {
    return (<div>home
      <FormattedMessage id="languages.zh-Hans"/>
    </div>)
  }
}

export default Widget