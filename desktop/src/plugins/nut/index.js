import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersConfirm from './users/Confirm'
import UsersUnlock from './users/Unlock'
import UsersForgotPassword from './users/ForgotPassword'

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
  }, {
    name: 'users.confirm',
    path: '/users/confirm',
    component: UsersConfirm
  }, {
    name: 'users.unlock',
    path: '/users/unlock',
    component: UsersUnlock
  }, {
    name: 'users.forgot-password',
    path: '/users/forgot-password',
    component: UsersForgotPassword
  }
]
