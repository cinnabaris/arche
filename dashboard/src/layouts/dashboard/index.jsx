import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'
import Exception from 'ant-design-pro/lib/Exception'

import Header from './Header'
import Footer from '../Footer'
import {createLoading} from '../'
import {reload as reloadAuthorized, TOKEN} from '../../Authorized'

class Widget extends Component {
  componentDidMount() {
    console.log('init')
    reloadAuthorized(localStorage.getItem(TOKEN))
  }
  render() {
    const {match} = this.props
    return (<div>
      <Header/>
      <Switch>
        <Route exact={true} path={`${match.url}/users/logs`} component={createLoading(() => import ('../../routes/users/Logs'))}/>
        <Route exact={true} path={`${match.url}/users/profile`} component={createLoading(() => import ('../../routes/users/Profile'))}/>
        <Route exact={true} path={`${match.url}/survey/forms/new`} component={createLoading(() => import ('../../routes/survey/forms/New'))}/>
        <Route component={() => (<Exception type="404"/>)}/>
      </Switch>
      <Footer/>
    </div>);
  }
}

export default Widget;
