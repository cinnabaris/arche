import React, {
  Component
} from 'react'
import {
  FormattedMessage
} from 'react-intl'

import Layout from "../../../components/layouts/Table"
import Authorized from '../../../components/Authorized'
import {
  failed,
  client
} from '../../../utils/request'
import {
  is_administrator
} from '../../../utils/authorized'


class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.name"/>),
        dataIndex: 'name',
        width: 280,
        key: 'name'
      }, {
        title: (<FormattedMessage id="attributes.content"/>),
        dataIndex: 'value',
        key: 'value'
      }]
    }
  }
  componentDidMount() {
    client().request(`query list{
        getSiteStatus {
          name,
          value
        }
      }`, {}).then((rst) => {
      this.setState({
        items: rst.getSiteStatus
      })
    }).catch(failed)
  }
  render() {
    const {
      items,
      columns
    } = this.state
    return (<Authorized check={is_administrator}>
      <Layout rowKey="name" title={{
        id: "nut.admin.site.status.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}

export default Widget;