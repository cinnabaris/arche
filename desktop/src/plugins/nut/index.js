import Home from './Home'

import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersLogs from './users/Logs'

export default {
  routes: [
    {
      path: '/users/sign-in',
      name: 'users.sign-in',
      component: UsersSignIn
    }, {
      path: '/users/sign-up',
      name: 'users.sign-up',
      component: UsersSignUp
    }, {
      path: '/users/logs',
      name: 'users.logs',
      component: UsersLogs
    }, {
      path: '/',
      name: 'home',
      component: Home
    }
  ]
}
