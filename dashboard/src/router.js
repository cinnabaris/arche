import {ALL, ADMIN} from './Authorized'

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
    hidden: true,
    children: [
      {
        path: "/sign-in",
        component: () => import ('./pages/users/SignIn')
      }, {
        path: "/sign-up",
        component: () => import ('./pages/users/SignUp')
      }, {
        path: "/confirm",
        component: () => import ('./pages/users/Confirm')
      }, {
        path: "/unlock",
        component: () => import ('./pages/users/Unlock')
      }, {
        path: "/forgot-password",
        component: () => import ('./pages/users/ForgotPassword')
      }, {
        path: "/confirm/:token",
        component: () => import ('./pages/users/ConfirmToken')
      }, {
        path: "/unlock/:token",
        component: () => import ('./pages/users/UnlockToken')
      }, {
        path: "/reset-password/:token",
        component: () => import ('./pages/users/ResetPassword')
      }
    ]
  }, {
    path: '/users',
    icon: 'user',
    label: {
      id: 'nut.users.dashboard.title'
    },
    authority: ALL,
    children: [
      {
        path: "/logs",
        component: () => import ('./pages/users/Logs'),
        label: {
          id: 'nut.users.logs.title'
        }
      }, {
        path: "/profile",
        component: () => import ('./pages/users/Profile'),
        label: {
          id: 'nut.users.profile.title'
        }
      }, {
        path: "/change-password",
        component: () => import ('./pages/users/ChangePassword'),
        label: {
          id: 'nut.users.change-password.title'
        }
      }
    ]
  }, {
    path: '/admin',
    icon: 'setting',
    label: {
      id: 'nut.admin.dashboard.title'
    },
    authority: [ADMIN],
    children: [
      {
        path: "/locales",
        component: () => import ('./pages/admin/locales/Index'),
        label: {
          id: 'nut.admin.locales.index.title'
        }
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

export const menus = items.filter((it) => it.children && !it.hidden).map((it) => {
  return {
    authority: it.authority,
    icon: it.icon,
    label: it.label,
    key: it.path,
    children: it.children.filter((it) => !it.hidden).map((jt) => {
      return {
        key: it.path + jt.path,
        label: jt.label,
        authority: jt.authority
          ? jt.authority
          : it.authority
      }
    })
  }
})
