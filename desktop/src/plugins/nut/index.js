import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'

export default {
  menus: [],
  routes: [
    {
      path: '/users/sign-in',
      name: 'users.sign-in',
      component: UsersSignIn
    }, {
      path: '/users/sign-up',
      name: 'users.sign-up',
      component: UsersSignUp
    }
  ]
}
