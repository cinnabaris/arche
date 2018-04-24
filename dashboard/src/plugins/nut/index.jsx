import {USER, ADMIN} from '../../auth'

const AdminCardsForm = import ('./admin/cards/Form')
const AdminLinksForm = import ('./admin/links/Form')
const AdminFriendLinksForm = import ('./admin/friend-links/Form')
const AdminLocalesForm = import ('./admin/locales/Form')

export default {
  routes: [
    {
      path: "/",
      component: import ("./Home")
    }, {
      path: "/users/sign-in",
      component: import ("./users/SignIn")
    }, {
      path: "/users/sign-up",
      component: import ("./users/SignUp")
    }, {
      path: "/users/confirm",
      component: import ("./users/Confirm")
    }, {
      path: "/users/unlock",
      component: import ("./users/Unlock")
    }, {
      path: "/users/forgot-password",
      component: import ("./users/ForgotPassword")
    }, {
      path: "/users/reset-password/:token",
      component: import ("./users/ResetPassword")
    }, {
      path: "/users/logs",
      component: import ("./users/Logs")
    }, {
      path: "/users/profile",
      component: import ("./users/Profile")
    }, {
      path: "/users/change-password",
      component: import ("./users/ChangePassword")
    }, {
      path: "/leave-words/new",
      component: import ("./leave-words/New")
    }, {
      path: "/admin/site/status",
      component: import ("./admin/site/Status")
    }, {
      path: "/admin/site/info",
      component: import ("./admin/site/Info")
    }, {
      path: "/admin/site/author",
      component: import ("./admin/site/Author")
    }, {
      path: "/admin/site/seo",
      component: import ("./admin/site/Seo")
    }, {
      path: "/admin/site/smtp",
      component: import ("./admin/site/Smtp")
    }, {
      path: "/admin/site/home",
      component: import ("./admin/site/Home")
    }, {
      path: "/admin/users",
      component: import ("./admin/users/Index")
    }, {
      path: "/admin/leave-words",
      component: import ("./admin/leave-words/Index")
    }, {
      path: "/admin/locales/edit/:id",
      component: AdminLocalesForm
    }, {
      path: "/admin/locales/new",
      component: AdminLocalesForm
    }, {
      path: "/admin/locales",
      component: import ("./admin/locales/Index")
    }, {
      path: "/admin/friend-links/edit/:id",
      component: AdminFriendLinksForm
    }, {
      path: "/admin/friend-links/new",
      component: AdminFriendLinksForm
    }, {
      path: "/admin/friend-links",
      component: import ("./admin/friend-links/Index")
    }, {
      path: "/admin/links/edit/:id",
      component: AdminLinksForm
    }, {
      path: "/admin/links/new",
      component: AdminLinksForm
    }, {
      path: "/admin/links",
      component: import ("./admin/links/Index")
    }, {
      path: "/admin/cards/edit/:id",
      component: AdminCardsForm
    }, {
      path: "/admin/cards/new",
      component: AdminCardsForm
    }, {
      path: "/admin/cards",
      component: import ("./admin/cards/Index")
    }, {
      path: "/attachments",
      component: import ("./attachments/Index")
    }
  ],
  menus: [
    {
      icon: "user",
      label: "nut.self.title",
      href: "personal",
      roles: [
        USER, ADMIN
      ],
      items: [
        {
          label: "nut.users.logs.title",
          href: "/users/logs"
        }, {
          label: "nut.users.profile.title",
          href: "/users/profile"
        }, {
          label: "nut.users.change-password.title",
          href: "/users/change-password"
        }, {
          label: "nut.attachments.index.title",
          href: "/attachments"
        }
      ]
    }, {
      icon: "setting",
      label: "nut.settings.title",
      href: "settings",
      roles: [ADMIN],
      items: [
        {
          label: "nut.admin.site.status.title",
          href: "/admin/site/status"
        }, {
          label: "nut.admin.site.info.title",
          href: "/admin/site/info"
        }, {
          label: "nut.admin.site.author.title",
          href: "/admin/site/author"
        }, {
          label: "nut.admin.site.seo.title",
          href: "/admin/site/seo"
        }, {
          label: "nut.admin.site.smtp.title",
          href: "/admin/site/smtp"
        }, {
          label: "nut.admin.site.home.title",
          href: "/admin/site/home"
        }, {
          label: "nut.admin.links.index.title",
          href: "/admin/links"
        }, {
          label: "nut.admin.cards.index.title",
          href: "/admin/cards"
        }, {
          label: "nut.admin.locales.index.title",
          href: "/admin/locales"
        }, {
          label: "nut.admin.friend-links.index.title",
          href: "/admin/friend-links"
        }, {
          label: "nut.admin.leave-words.index.title",
          href: "/admin/leave-words"
        }, {
          label: "nut.admin.users.index.title",
          href: "/admin/users"
        }
      ]
    }
  ]
}
