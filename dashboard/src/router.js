import {ADMIN, MEMBER} from './Authorized'

export const Application = () => import ('./layouts/Application')
export const Dashboard = () => import ('./layouts/dashboard')

const ALL = [ADMIN, MEMBER]

const items = [
  {
    path: "/",
    component: () => import ('./pages/Home')
  },
  //{
  //   path: "/install",
  //   component: () => import ('./pages/Install')
  // },
  {
    path: '/users',
    layout: Application,
    children: [
      {
        path: "/sign-in",
        component: () => import ('./pages/users/SignIn'),
        args: {
          title: 'nut.users.sign-in.title'
        }
      }, {
        path: "/sign-up",
        component: () => import ('./pages/users/SignUp'),
        args: {
          title: 'nut.users.sign-up.title'
        }
      }, {
        path: "/logs",
        layout: Dashboard,
        component: () => import ('./pages/users/Logs'),
        authority: ALL,
        args: {
          title: 'nut.users.logs.title'
        }
      }, {
        path: "/profile",
        layout: Dashboard,
        component: () => import ('./pages/users/Profile'),
        authority: ALL,
        args: {
          title: 'nut.users.profile.title'
        }
      }
    ]
  }
  // {
  //   path: "/users/confirm",
  //   component: () => import ('./pages/users/Confirm')
  // }, {
  //   path: "/users/unlock",
  //   component: () => import ('./pages/users/Unlock')
  // }, {
  //   path: "/users/forgot-password",
  //   component: () => import ('./pages/users/ForgotPassword')
  // }, {
  //   path: "/users/confirm/:token",
  //   component: () => import ('./pages/users/ConfirmToken')
  // }, {
  //   path: "/users/unlock/:token",
  //   component: () => import ('./pages/users/UnlockToken')
  // }, {
  //   path: "/users/reset-password/:token",
  //   component: () => import ('./pages/users/ResetPassword')
  // }, {
  //   path: "/leave-words/new",
  //   component: () => import ('./pages/leave-words/New')
  // }
]

export const routes = items.reduce((ar, it) => {
  return ar.concat(
    it.children
    ? it.children.map((jt) => {
      return {
        layout: jt.layout
          ? jt.layout
          : it.layout,
        authority: jt.authority
          ? jt.authority
          : it.authority,
        path: it.path + jt.path,
        component: jt.component,
        args: jt.args
      }
    })
    : [it])
}, [])
