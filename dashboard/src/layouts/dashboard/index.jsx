import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'
import Exception from 'ant-design-pro/lib/Exception'

import Header from './Header'
import Footer from './Footer'
import {createLoading} from '../'

class Widget extends Component {
  render() {
    const {match} = this.props
    return (<div>
      <Header/>
      <Switch>
        <Route exact={true} path={`${match.url}/survey/forms/new`} component={createLoading(() => import ('../../routes/survey/forms/New'))}/>
        <Route component={() => (<Exception type="404"/>)}/>
      </Switch>
      <Footer/>
    </div>);
  }
}

export default Widget;
