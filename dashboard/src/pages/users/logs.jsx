import React, {
  Component
} from 'react'
import {
  FormattedMessage
} from 'react-intl'

import Layout from "../../components/layouts/Table"
import Authorized from '../../components/Authorized'
import {
  failed,
  client
} from '../../utils/request'
import {
  show as moment
} from '../../utils/moment'
import {
  is_sign_in
} from '../../utils/authorized'


class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.created-at"/>),
        render: (text, record) => moment(record.createdAt),
        width: 280,
        key: 'createdAt'
      }, {
        title: (<FormattedMessage id="attributes.ip"/>),
        dataIndex: 'ip',
        width: 150,
        key: 'ip'
      }, {
        title: (<FormattedMessage id="attributes.content"/>),
        dataIndex: 'message',
        key: 'message'
      }]
    }
  }
  componentDidMount() {
    client().request(`query logs{
        listUserLog {
          id,
          ip,
          message,
          createdAt
        }
      }`, {}).then((rst) => {
      this.setState({
        items: rst.listUserLog
      })
    }).catch(failed)
  }
  render() {
    const {
      items,
      columns
    } = this.state
    return (<Authorized check={is_sign_in}>
      <Layout rowKey="id" title={{
        id: "nut.users.logs.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}

export default Widget;