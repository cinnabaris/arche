import UsersLogs from './users/Logs'
import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersConfirm from './users/Confirm'
import UsersUnlock from './users/Unlock'
import UsersForgotPassword from './users/ForgotPassword'
import UsersConfirmToken from './users/ConfirmToken'
import UsersUnlockToken from './users/UnlockToken'
import UsersResetPassword from './users/ResetPassword'

import NewLeaveWord from './leave-words/New'

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
    name: 'users.logs',
    path: '/users/logs',
    component: UsersLogs
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
    name: 'users.confirm.token',
    path: '/users/confirm/:token',
    component: UsersConfirmToken
  }, {
    name: 'users.unlock.token',
    path: '/users/unlock/:token',
    component: UsersUnlockToken
  }, {
    name: 'users.unlock',
    path: '/users/unlock',
    component: UsersUnlock
  }, {
    name: 'users.forgot-password',
    path: '/users/forgot-password',
    component: UsersForgotPassword
  }, {
    name: 'users.reset-password.token',
    path: '/users/reset-password/:token',
    component: UsersResetPassword
  }, {
    name: 'leave-words.new',
    path: '/leave-words/new',
    component: NewLeaveWord
  }
]
