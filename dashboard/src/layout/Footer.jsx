import React, {
  Component
} from 'react'
import GlobalFooter from 'ant-design-pro/lib/GlobalFooter'
import {
  FormattedMessage
} from 'react-intl'
import {
  Icon
} from 'antd'

const Copyright = () => (<div>
  <Icon type="copyright"/>
  <FormattedMessage id="site.copyright"/>
</div>)

class Widget extends Component {
  constructor(props) {
    super(props)
    const home = 'https://github.com/cinnabaris/arche'
    this.state = {
      items: [{
        key: 'help',
        title: <FormattedMessage id="footer.help"/>,
        href: `${home}/blob/master/README.md`
      }, {
        key: 'github',
        title: <Icon type="github"/>,
        href: home,
        blankTarget: true
      }, {
        key: 'license',
        title: <FormattedMessage id="footer.license"/>,
        href: `${home}/blob/master/LICENSE`,
        blankTarget: true
      }]
    }
  }
  render() {
    return (<GlobalFooter
      links={this.state.items}
      copyright={<Copyright/>}/>)
  }
}

export default Widget