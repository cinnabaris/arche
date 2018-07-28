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
  List,
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

class Widget extends Component {
  constructor(props) {
      super(props)
      this.state = {
          items: [],
          columns: [{
                title: (<FormattedMessage id="attributes.username"/>),
                render: (text, record) => `${record.nickName}[${record.realName}]`,
                width: 320,
                key: 'user'
              }, {
                title: (<FormattedMessage id="attributes.gender"/>),
                render: (text, record) => <FormattedMessage id={`attributes.gender-${record.gender}`}/>,
                width: 80,
                key: 'gender'
              }, {
                title: (<FormattedMessage id="attributes.contact"/>),
                render: (text, record) => (<List
                  size="small"
                  bordered
                  dataSource={['phone', 'email', 'address', 'line', 'wechat', 'skype', 'weibo', 'facebook'].map((it)=>(<span key={it}><FormattedMessage id={`nut.attributes.member.${it}`}/>: {record[it]}</span>))}
                  renderItem={it => (<List.Item>{it}</List.Item>)}/>),
        key: 'contact'
      }, {
        title: (<span><FormattedMessage id="buttons.operator"/>&nbsp;<Button onClick={()=>router.push(`/admin/members/new`)} size="small" shape="circle" type="primary" icon="file-add"/></span>),
        width: 120,
        render: (text, record) => (<span>
          <Button size="small" onClick={()=>router.push(`/admin/members/${record.id}/edit`)} shape="circle" type="dashed" icon="edit"/>
          <Popconfirm title={<FormattedMessage id="flashes.confirm-to-remove" values={{name:record.nickName}} />} onConfirm={(e) => this.handleRemove(record.id)}>
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
          removeMember(id: $id) {
            createdAt
          }
        }`, {
      id
    }).then(() => {
      message.info(formatMessage({
        id: "flashes.success"
      }))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({items})
    }).catch(failed)
  }
  componentDidMount() {
    client().request(`query list{
        listMember{
          id, nickName, realName, gender, birthday, phone, email, address, line, wechat, skype, weibo, facebook
        }
      }`).then((rst) => {
      this.setState({
        items: rst.listMember
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
        id: "nut.admin.members.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)