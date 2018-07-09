import React, {Component} from 'react'
import GlobalFooter from 'ant-design-pro/lib/GlobalFooter'
import {FormattedMessage, injectIntl} from 'react-intl'
import {Icon} from 'antd'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [
        {
          key: '帮助',
          title: '帮助',
          href: 'https://github.com/cinnabaris/arche/blob/master/README.md'
        }, {
          key: 'github',
          title: <Icon type="github"/>,
          href: 'https://github.com/cinnabaris/arche',
          blankTarget: true
        }, {
          key: '条款',
          title: '条款',
          href: 'https://github.com/cinnabaris/arche/blob/master/LICENSE',
          blankTarget: true
        }
      ]
    }
  }
  render() {
    return (<GlobalFooter links={this.state.items} copyright={(<div><Icon type="copyright"/>
        <FormattedMessage id="site.copyright"/></div>)}/>);
  }
}

export default injectIntl(Widget);
