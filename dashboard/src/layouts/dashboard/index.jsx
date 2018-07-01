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
        <Route exact={true} path={`${match.url}/survey/forms/new`} component={createLoading(() => import ('../../routes/survey/forms/New'))}/>
        <Route component={NotFound}/>
      </Switch>
      <Footer/>
    </div>);
  }
}

export default Widget;
