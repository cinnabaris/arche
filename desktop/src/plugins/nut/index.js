import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'

export default[
  {
    name: 'user.sign-in',
    path: '/users/sign-in',
    component: UsersSignIn
  }, {
    name: 'user.sign-up',
    path: '/users/sign-up',
    component: UsersSignUp
  }
]
