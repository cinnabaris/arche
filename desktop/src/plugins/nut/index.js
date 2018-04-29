import Home from './Home'

import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'

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
      path: '/',
      name: 'home',
      component: Home
    }
  ]
}
