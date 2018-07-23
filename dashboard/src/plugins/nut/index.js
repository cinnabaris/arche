export default {
  routes: [{
      path: '/',
      component: () =>
        import ('./Home')
    },
    {
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