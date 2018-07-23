import Home from './Home'
import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'

export default {
  routes: [{
      path: '/',
      component: Home
    },
    {
      path: '/users/sign-in',
      component: UsersSignIn
    },
    {
      path: '/users/sign-up',
      component: UsersSignUp
    }
  ]
}