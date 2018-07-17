import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import Install from './Install'
import Home from './Home'

export default[
  {
    name: 'home',
    path: '/',
    component: Home
  }, {
    name: 'install',
    path: '/install',
    component: Install
  }, {
    name: 'users.sign-in',
    path: '/users/sign-in',
    component: UsersSignIn
  }, {
    name: 'users.sign-up',
    path: '/users/sign-up',
    component: UsersSignUp
  }
]
