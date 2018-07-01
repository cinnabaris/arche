import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'
import {
  Layout,
  Row,
  List,
  Divider,
  Col,
  Avatar,
  Card,
  Button
} from 'antd'
import Exception from 'ant-design-pro/lib/Exception'

import {createLoading} from '../'
import Header from './Header'
import Footer from './Footer'

const {Content} = Layout;

class Widget extends Component {
  constructor(props) {
    super(props);
    this.state = {
      links: [
        {
          icon: 'login',
          to: 'users.sign-in'
        }, {
          icon: 'user-add',
          to: 'users.sign-up'
        }, {
          icon: 'retweet',
          to: 'users.forgot-password'
        }, {
          icon: 'check',
          to: 'users.confirm'
        }, {
          icon: 'unlock',
          to: 'users.unlock'
        }, {
          icon: 'message',
          to: 'users.leave-message'
        }
      ]
    };
  }
  render() {
    const {match} = this.props
    return (<Layout>
      <Header/>
      <Content style={{
          padding: '0 24px',
          minHeight: 280
        }}>
        <Row gutter={16}>
          <Col xs={{
              span: 22,
              offset: 1
            }} lg={{
              span: 12,
              offset: 6
            }}>
            <br/>
            <Card title="Card title" extra={(<a href="/" target="_blank">
                More</a>)}>
              <Switch>
                <Route exact={true} path={`${match.url}/sign-in`} component={createLoading(() => import ('../../routes/users/SignIn'))}/>
                <Route exact={true} path={`${match.url}/sign-up`} component={createLoading(() => import ('../../routes/users/SignUp'))}/>
                <Route component={() => (<Exception type="404"/>)}/>
              </Switch>
              <Divider/>
              <Button type="primary">Primary</Button>
            </Card>
            <br/>
            <List bordered={true} size="small" itemLayout="horizontal" dataSource={this.state.links} renderItem={it => (<List.Item>
                <List.Item.Meta avatar={(<Avatar icon={it.icon}/>)} description={it.to}/>
              </List.Item>)}/>
          </Col>
        </Row>

      </Content>
      <Footer/>
    </Layout>);
  }
}

export default Widget;
