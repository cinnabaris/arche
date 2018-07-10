import {ALL} from './Authorized'

const items = [
  {
    path: "/",
    component: () => import ('./pages/Home'),
    hidden: true
  }, {
    path: "/install",
    component: () => import ('./pages/Install'),
    hidden: true
  }, {
    path: '/users',
    children: [
      {
        path: "/logs",
        component: () => import ('./pages/users/Logs'),
        authority: ALL
      }, {
        path: "/profile",
        component: () => import ('./pages/users/Profile'),
        authority: ALL
      }, {
        path: "/change-password",
        component: () => import ('./pages/users/ChangePassword'),
        authority: ALL
      }, {
        path: "/sign-in",
        component: () => import ('./pages/users/SignIn'),
        hidden: true
      }, {
        path: "/sign-up",
        component: () => import ('./pages/users/SignUp'),
        hidden: true
      }, {
        path: "/confirm",
        component: () => import ('./pages/users/Confirm'),
        hidden: true
      }, {
        path: "/unlock",
        component: () => import ('./pages/users/Unlock'),
        hidden: true
      }, {
        path: "/forgot-password",
        component: () => import ('./pages/users/ForgotPassword'),
        hidden: true
      }, {
        path: "/confirm/:token",
        component: () => import ('./pages/users/ConfirmToken'),
        hidden: true
      }, {
        path: "/unlock/:token",
        component: () => import ('./pages/users/UnlockToken'),
        hidden: true
      }, {
        path: "/reset-password/:token",
        component: () => import ('./pages/users/ResetPassword'),
        hidden: true
      }
    ]
  }, {
    path: "/leave-words/new",
    component: () => import ('./pages/leave-words/New'),
    hidden: true
  }
]

export const routes = items.reduce((ar, it) => {
  return ar.concat(
    it.children
    ? it.children.map((jt) => {
      return {
        authority: jt.authority
          ? jt.authority
          : it.authority,
        path: it.path + jt.path,
        component: jt.component
      }
    })
    : [it])
}, [])

export const menus = items.filter((it) => it.children).reduce((ar, it) => {
  return ar.concat(
    it.children
    ? it.children.map((jt) => {
      return {
        authority: jt.authority
          ? jt.authority
          : it.authority,
        path: it.path + jt.path,
        component: jt.component
      }
    })
    : [it])
}, [])
