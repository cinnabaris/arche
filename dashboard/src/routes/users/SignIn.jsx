import React, {Component} from 'react'
import DocumentTitle from 'react-document-title'

class Widget extends Component {

  render() {
    return (<DocumentTitle title='My Web App'>
      <div>
        users.sign-in
      </div>
    </DocumentTitle>);
  }
}

export default Widget;
