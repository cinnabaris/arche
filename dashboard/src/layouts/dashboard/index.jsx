import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'
import Exception from 'ant-design-pro/lib/Exception'

import Header from './Header'
import Footer from '../Footer'
import createLoading from '../../loading'
import {reload as reloadAuthorized, TOKEN} from '../../Authorized'

class Widget extends Component {
  componentDidMount() {
    console.log('init dashboard')
    if (false) {
      reloadAuthorized(localStorage.getItem(TOKEN))
    }
  }
  render() {
    const {match, children} = this.props
    return (<div>
      <Header/> {children}
      <Footer/>
    </div>);
  }
}

export default Widget;
