import Home from './Home'
import UsersSignIn from './users/SignIn'
import NewLeaveWord from './leave-words/New'

const routes = [
  {
    name: 'leave-word.new',
    path: '/leave-words/new',
    component: NewLeaveWord
  }, {
    name: 'users.sign-in',
    path: '/users/sign-in',
    component: UsersSignIn
  }, {
    name: 'home',
    path: '/',
    component: Home
  }
]

export default routes
