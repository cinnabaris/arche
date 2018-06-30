export default {
  routes: [
    {
      component: () => import ('./users/SignIn'),
      path: '/users/sign-in',
      name: 'users.sign-in'
    }, {
      component: () => import ('./users/SignUp'),
      path: '/users/sign-up',
      name: 'users.sign-up'
    }, {
      component: () => import ('./users/Confirm'),
      path: '/users/confirm',
      name: 'users.confirm'
    }, {
      component: () => import ('./users/Unlock'),
      path: '/users/unlock',
      name: 'users.unlock'
    }, {
      component: () => import ('./users/ForgotPassword'),
      path: '/users/forgot-password',
      name: 'users.forgot-password'
    }, {
      component: () => import ('./users/ResetPassword'),
      path: '/users/reset-password',
      name: 'users.reset-password'
    }, {
      component: () => import ('./Install'),
      path: '/install',
      name: 'install'
    }, {
      component: () => import ('./Home'),
      path: '/',
      name: 'home'
    }
  ],
  menus: []
}
