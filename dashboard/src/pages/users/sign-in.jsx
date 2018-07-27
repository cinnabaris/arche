import React, {
  Component
} from 'react'

import Layout from "../../components/layouts/UsersSharedLinks"

class Widget extends Component {
  componentDidMount() {
    console.log('init sign-in')
  }
  render() {
    return (<Layout title="nut.users.sign-in.title">
      sign in
    </Layout>)
  }
}

export default Widget