import React, {
  Component
} from 'react'
import {
  intlShape,
  injectIntl,
  FormattedMessage
} from 'react-intl'
import router from 'umi/router'
import {
  Button,
  Popconfirm,
  message
} from 'antd'

import Layout from "../../../components/layouts/Table"
import Authorized from '../../../components/Authorized'
import {
  failed,
  client
} from '../../../utils/request'
import {
  show as moment
} from '../../../utils/moment'
import {
  is_caring_manager,
  is_sign_in
} from '../../../utils/authorized'


class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.updated-at"/>),
        render: (text, record) => moment(record.updatedAt),
        width: 280,
        key: 'updatedAt'
      }, {
        title: (<FormattedMessage id="attributes.username"/>),
        dataIndex: 'name',
        width: 150,
        key: 'name'
      }, {
        title: (<FormattedMessage id="attributes.content"/>),
        render: (text, record) => (<span>
          <FormattedMessage id={`caring.attributes.topic.tag-${record.tag}`}/>
          [<FormattedMessage id={`caring.attributes.topic.status-${record.status}`}/>]
        </span>),
        key: 'tag'
      }, {
        title: (<span>
          <FormattedMessage id="buttons.operator"/>
          &nbsp;
          <Authorized hidden check={is_caring_manager}>
            <Button onClick={()=>router.push(`/caring/topics/new`)} size="small" shape="circle" type="primary" icon="file-add"/>
          </Authorized>
        </span>),
        width: 120,
        render: (text, record) => (record.editable ? (<span>
          <Button size="small" onClick={()=>router.push(`/caring/topics/${record.id}/edit`)} shape="circle" type="dashed" icon="edit"/>
          <Popconfirm title={<FormattedMessage id="flashes.confirm-to-remove" values={{name:record.name}} />} onConfirm={(e) => this.handleRemove(record.id)}>
            <Button size="small" shape="circle" type="danger" icon="delete"/>
          </Popconfirm>
        </span>) : <span/>),
        key: 'operator'
      }]
    }
  }
  handleRemove = (id) => {
    const {
      formatMessage
    } = this.props.intl
    client().request(`mutation form($id: String!){
          removeCaringTopic(id: $id) {
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
        listCaringTopic {
          id, name, tag, editable, updatedAt, status
        }
      }`, {}).then((rst) => {
      this.setState({
        items: rst.listCaringTopic
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
        id: "caring.topics.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)