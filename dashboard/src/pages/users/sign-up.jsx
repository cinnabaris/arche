import React, {
  Component
} from 'react'

import Layout from "../../components/layouts/UsersSharedLinks"

class Widget extends Component {
  componentDidMount() {
    console.log('init sign-up')
  }
  render() {
    return (<Layout title="nut.users.sign-up.title">
      sign up
    </Layout>)
  }
}

export default Widget