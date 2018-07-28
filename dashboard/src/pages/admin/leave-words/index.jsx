import React, {
  Component
} from 'react'
import {
  FormattedMessage,
  intlShape,
  injectIntl
} from 'react-intl'
import {
  Button,
  Popconfirm,
  message
} from 'antd';

import Layout from "../../../components/layouts/Table"
import Authorized from '../../../components/Authorized'
import {
  failed,
  client
} from '../../../utils/request'
import {
  is_administrator
} from '../../../utils/authorized'
import {
  show as moment
} from '../../../utils/moment'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.updated-at"/>),
        render: (text, record) => moment(record.createdAt),
        width: 280,
        key: 'updatedAt'
      }, {
        title: (<FormattedMessage id="attributes.content"/>),
        render: (text, record) => <pre>{record.body}</pre>,
        key: 'body'
      }, {
        title: (<FormattedMessage id="buttons.operator"/>),
        width: 120,
        render: (text, record) => (<span>
          <Popconfirm title={<FormattedMessage id="flashes.confirm-to-remove" values={{name:record.id}} />} onConfirm={(e) => this.handleRemove(record.id)}>
            <Button size="small" shape="circle" type="danger" icon="delete"/>
          </Popconfirm>
        </span>),
        key: 'operator'
      }]
    }
  }
  handleRemove = (id) => {
    const {
      formatMessage
    } = this.props.intl
    client().request(`mutation form($id: String!){
          removeLeaveWord(id: $id) {
            createdAt
          }
        }`, {
      id
    }).then(() => {
      message.info(formatMessage({
        id: "flashes.success"
      }))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({
        items
      })
    }).catch(failed)
  }
  componentDidMount() {
    client().request(`query list{
        listLeaveWord{
          id, body, createdAt
        }
      }`, {}).then((rst) => {
      this.setState({
        items: rst.listLeaveWord
      })
    }).catch(failed)
  }
  render() {
    const {
      items,
      columns
    } = this.state
    return (<Authorized check={is_administrator}>
      <Layout rowKey="id" title={{
        id: "nut.admin.leave-words.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)