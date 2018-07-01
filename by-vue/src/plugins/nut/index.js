import Home from './Home'

import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersConfirm from './users/Confirm'
import UsersUnlock from './users/Unlock'
import UsersForgotPassword from './users/ForgotPassword'

import NewLeaveWord from './leave-words/New'

const routes = [
  {
    name: 'users.sign-in',
    path: '/users/sign-in',
    component: UsersSignIn
  }, {
    name: 'users.sign-up',
    path: '/users/sign-up',
    component: UsersSignUp
  }, {
    name: 'users.confirm',
    path: '/users/UsersConfirm',
    component: UsersConfirm
  }, {
    name: 'users.unlock',
    path: '/users/UsersUnlock',
    component: UsersUnlock
  }, {
    name: 'users.forgot-password',
    path: '/users/UsersForgotPassword',
    component: UsersForgotPassword
  }, {
    name: 'leave-words.new',
    path: '/leave-words/new',
    component: NewLeaveWord
  }, {
    name: 'home',
    path: '/',
    component: Home
  }
]

export default routes
