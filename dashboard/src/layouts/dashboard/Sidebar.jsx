import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {Menu, Icon, Modal, message} from 'antd'
import {push} from 'react-router-redux'
import {Link} from 'react-router-dom'

import {signOut, toggleSideBar, selectSideBar} from '../../actions'
import {_delete} from '../../ajax'
import {Authorized, USER, ADMIN} from '../../auth'

import plugins from '../../plugins'

const SubMenu = Menu.SubMenu
const MenuItem = Menu.Item
const confirm = Modal.confirm

class Widget extends Component {
  handleOpenChange = (keys) => {
    const {toggleSideBar} = this.props
    toggleSideBar(keys)
  }
  handleMenu = ({key}) => {
    const {push, signOut, selectSideBar} = this.props
    const {formatMessage} = this.props.intl

    selectSideBar(key)

    switch (key) {
      case "users.sign-out":
        confirm({
          title: formatMessage({id: "helpers.are-you-sure"}),
          onOk() {
            _delete('/users/sign-out').then(() => {
              signOut()
              push('/users/sign-in')
              message.success(formatMessage({id: 'flash.success'}))
            }).catch(message.error)
          }
        });
        break
      default:
        break
    }
  }
  render() {
    const {bar} = this.props
    return (<Menu theme="dark" mode="inline" defaultSelectedKeys={bar.selected} defaultOpenKeys={bar.open} onClick={this.handleMenu} onOpenChange={this.handleOpenChange}>
      {
        plugins.menus.map((it) => Authorized.check(it.roles, (<SubMenu key={it.href} title={(<span >
            <Icon type={it.icon}/>
            <FormattedMessage id={it.label}/>
          </span>)}>
          {
            it.items.map((l) => Authorized.check(l.roles || it.roles, (<MenuItem key={`${it.href}-${l.href}`}>
              <Link to={l.href}>
                <FormattedMessage id={l.label}/>
              </Link>
            </MenuItem>)))
          }
        </SubMenu>)))
      }
      {
        Authorized.check([
          ADMIN, USER
        ], (<MenuItem key='users.sign-out'>
          <Icon type='logout'/>
          <FormattedMessage id='nut.users.sign-out.title'/>
        </MenuItem>))
      }
    </Menu>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired,
  signOut: PropTypes.func.isRequired,
  toggleSideBar: PropTypes.func.isRequired,
  selectSideBar: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired,
  bar: PropTypes.object.isRequired
}

const WidgetI = injectIntl(Widget)
export default connect(state => ({user: state.currentUser, bar: state.sideBar}), {push, signOut, toggleSideBar, selectSideBar})(WidgetI)
