import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'

import Header from './Header'
import Footer from './Footer'
import {createLoading} from '../'
import NotFound from '../../routes/NotFound'

class Widget extends Component {
  render() {
    const {match} = this.props
    return (<div>
      <Header/>
      <Switch>
        <Route exact={true} path={`${match.url}/sign-in`} component={createLoading(() => import ('../../routes/users/SignIn'))}/>
        <Route exact={true} path={`${match.url}/sign-up`} component={createLoading(() => import ('../../routes/users/SignUp'))}/>
        <Route component={NotFound}/>
      </Switch>
      <Footer/>
    </div>);
  }
}

export default Widget;
