import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'

import Header from './Header'
import Footer from './Footer'
import {createLoading} from '../'

class Widget extends Component {
  render() {
    return (<div>
      <Header/>
      <Route exact={true} path="/survey/forms/new" component={createLoading(() => import ('../../routes/survey/forms/New'))}/>
      <Footer/>
    </div>);
  }
}

export default Widget;
