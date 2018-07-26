export default {
  menus: [],
  routes: [{
      path: '/users/sign-in',
      component: () =>
        import ('./users/SignIn')
    },
    {
      path: '/users/sign-up',
      component: () =>
        import ('./users/SignUp')
    }
  ]
}