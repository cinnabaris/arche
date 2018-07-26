import React, {Component} from 'react'
import {FormattedMessage} from 'react-intl'

import {client, USERS_LOGS, failed} from '../../../request'
import Table from '../../../components/Table'
import {Timestamp} from '../../../components/moment'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [
        {
          title: (<FormattedMessage id="attributes.createdAt"/>),
          render: (text, record) => (<Timestamp value={record.createdAt}/>),
          key: 'createdAt'
        }, {
          title: (<FormattedMessage id="attributes.ip"/>),
          dataIndex: 'ip',
          key: 'ip'
        }, {
          title: (<FormattedMessage id="attributes.message"/>),
          dataIndex: 'message',
          key: 'message'
        }
      ]
    }
  }
  componentDidMount() {
    client().request(USERS_LOGS).then((rst) => {
      this.setState({items: rst.listUserLog})
    }).catch(failed)
  }
  render() {
    const {items, columns} = this.state
    return (<Table title={{
        id: "nut.users.logs.title"
      }} items={items} columns={columns}/>)
  }
}

export default Widget;
