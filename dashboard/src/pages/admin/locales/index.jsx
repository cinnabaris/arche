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
  is_administrator
} from '../../../utils/authorized'
import {
  get as getLocale
} from '../../../utils/locale'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="nut.attributes.locale.code"/>),
        dataIndex: 'code',
        width: 320,
        key: 'code'
      }, {
        title: (<FormattedMessage id="attributes.content"/>),
        dataIndex: 'message',
        key: 'message'
      }, {
        title: (<span><FormattedMessage id="buttons.operator"/>&nbsp;<Button onClick={()=>router.push(`/admin/locales/new`)} size="small" shape="circle" type="primary" icon="file-add"/></span>),
        width: 120,
        render: (text, record) => (<span>
          <Button size="small" onClick={()=>router.push(`/admin/locales/${record.code}/edit`)} shape="circle" type="dashed" icon="edit"/>
          <Popconfirm title={<FormattedMessage id="flashes.confirm-to-remove" values={{name:record.code}} />} onConfirm={(e) => this.handleRemove(record.id)}>
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
          removeLocale(id: $id) {
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
    client().request(`query locales($lang: String!){
        listLocaleByLang(lang: $lang) {
          id, code, message
        }
      }`, {
      lang: getLocale()
    }).then((rst) => {
      this.setState({
        items: rst.listLocaleByLang
      })
    }).catch(failed)
  }
  render() {
    const {
      items,
      columns
    } = this.state
    return (<Authorized check={is_administrator}>
      <Layout rowKey="code" title={{
        id: "nut.admin.locales.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)