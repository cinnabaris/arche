import React, {
  Component
} from 'react'
import {
  FormattedMessage,
  intlShape,
  injectIntl
} from 'react-intl'
import router from 'umi/router'
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
  show as moment
} from '../../../utils/moment'
import {
  is_administrator
} from '../../../utils/authorized'

const show_sign_in = (at, ip) => `${at ? moment(at) : ''}[${ip ? ip : ''}]`

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.user"/>),
        render: (text, record) => `${record.name}\u003C${record.email}\u003E[${record.signInCount}]`,
        width: 320,
        key: 'user'
      }, {
        title: (<FormattedMessage id="nut.attributes.user.last-sign-in"/>),
        render: (text, record) => show_sign_in(record.lastSignInAt, record.lastSignInIp),
        key: 'last-sign-in'
      }, {
        title: (<FormattedMessage id="nut.attributes.user.current-sign-in"/>),
        render: (text, record) => show_sign_in(record.currentSignInAt, record.currentSignInIp),
        key: 'current-sign-in'
      }, {
        title: (<FormattedMessage id="buttons.operator"/>),
        width: 90,
        render: (text, record) => (<span>
          <Button size="small" onClick={()=>router.push(`/admin/users/${record.id}/policies`)} shape="circle" type="dashed" icon="edit"/>
          <Popconfirm title={<FormattedMessage id="nut.admin.users.index.lock.confirm" values={{name:record.name}} />} onConfirm={(e) => this.handleLock(record.id)}>
            <Button size="small" shape="circle" type="danger" icon="lock"/>
          </Popconfirm>
        </span>),
        key: 'operator'
      }]
    }
  }
  handleLock = (id) => {
    const {
      formatMessage
    } = this.props.intl
    client().request(`mutation form($id: String!){
        lockUser(id: $id) {
          createdAt
        }
      }`, {
      id
    }).then(() => {
      message.info(formatMessage({
        id: "flashes.success"
      }))
    }).catch(failed)
  }
  componentDidMount() {
    client().request(`query list{
        listUser{
          id, name, email, lastSignInAt, lastSignInIp, currentSignInAt, currentSignInIp, signInCount
        }
      }`).then((rst) => {
      this.setState({
        items: rst.listUser
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
        id: "nut.admin.users.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)