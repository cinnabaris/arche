export default[
  {
    path: "/",
    component: () => import ('./Home')
  }, {
    path: "/install",
    component: () => import ('./Install')
  }, {
    path: "/users/sign-in",
    component: () => import ('./users/SignIn')
  }, {
    path: "/users/sign-up",
    component: () => import ('./users/SignUp')
  }, {
    path: "/users/confirm",
    component: () => import ('./users/Confirm')
  }, {
    path: "/users/unlock",
    component: () => import ('./users/Unlock')
  }, {
    path: "/users/forgot-password",
    component: () => import ('./users/ForgotPassword')
  }, {
    path: "/users/confirm/:token",
    component: () => import ('./users/ConfirmToken')
  }, {
    path: "/users/unlock/:token",
    component: () => import ('./users/UnlockToken')
  }, {
    path: "/users/reset-password/:token",
    component: () => import ('./users/ResetPassword')
  }, {
    path: "/leave-words/new",
    component: () => import ('./leave-words/New')
  }
]
