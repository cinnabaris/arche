import UsersLogs from './users/Logs'
import UsersProfile from './users/Profile'
import UsersChangePassword from './users/ChangePassword'
import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersConfirm from './users/Confirm'
import UsersUnlock from './users/Unlock'
import UsersForgotPassword from './users/ForgotPassword'
import UsersConfirmToken from './users/ConfirmToken'
import UsersUnlockToken from './users/UnlockToken'
import UsersResetPassword from './users/ResetPassword'

import NewLeaveWord from './leave-words/New'

import Install from './Install'
import Home from './Home'

import AdminFriendLinksIndex from './admin/friend-links/Index'
import AdminFriendLinksForm from './admin/friend-links/Form'
import AdminCardsIndex from './admin/cards/Index'
import AdminCardsForm from './admin/cards/Form'
import AdminLinksIndex from './admin/links/Index'
import AdminLinksForm from './admin/links/Form'
import AdminLeaveWordsIndex from './admin/leave-words/Index'
import AdminLocalesIndex from './admin/locales/Index'
import AdminLocalesForm from './admin/locales/Form'
import AdminSiteStatus from './admin/site/Status'
import AdminSiteInfo from './admin/site/Info'
import AdminSiteAuthor from './admin/site/Author'
import AdminSiteSeo from './admin/site/Seo'
import AdminSiteSmtp from './admin/site/Smtp'

export default[
  {
    name: 'home',
    path: '/',
    component: Home
  }, {
    name: 'install',
    path: '/install',
    component: Install
  }, {
    name: 'admin.site.status',
    path: '/admin/site/status',
    component: AdminSiteStatus
  }, {
    name: 'admin.site.info',
    path: '/admin/site/info',
    component: AdminSiteInfo
  }, {
    name: 'admin.site.author',
    path: '/admin/site/author',
    component: AdminSiteAuthor
  }, {
    name: 'admin.site.seo',
    path: '/admin/site/seo',
    component: AdminSiteSeo
  }, {
    name: 'admin.site.smtp',
    path: '/admin/site/smtp',
    component: AdminSiteSmtp
  }, {
    name: 'admin.friend-links.index',
    path: '/admin/friend-links',
    component: AdminFriendLinksIndex
  }, {
    name: 'admin.friend-links.new',
    path: '/admin/friend-links/new',
    component: AdminFriendLinksForm
  }, {
    name: 'admin.friend-links.edit',
    path: '/admin/friend-links/:id/edit',
    component: AdminFriendLinksForm
  }, {
    name: 'admin.cards.index',
    path: '/admin/cards',
    component: AdminCardsIndex
  }, {
    name: 'admin.cards.new',
    path: '/admin/cards/new',
    component: AdminCardsForm
  }, {
    name: 'admin.cards.edit',
    path: '/admin/cards/:id/edit',
    component: AdminCardsForm
  }, {
    name: 'admin.links.index',
    path: '/admin/links',
    component: AdminLinksIndex
  }, {
    name: 'admin.links.new',
    path: '/admin/links/new',
    component: AdminLinksForm
  }, {
    name: 'admin.links.edit',
    path: '/admin/links/:id/edit',
    component: AdminLinksForm
  }, {
    name: 'admin.leave-words.index',
    path: '/admin/leave-words',
    component: AdminLeaveWordsIndex
  }, {
    name: 'admin.locales.index',
    path: '/admin/locales',
    component: AdminLocalesIndex
  }, {
    name: 'admin.locales.new',
    path: '/admin/locales/new',
    component: AdminLocalesForm
  }, {
    name: 'admin.locales.edit',
    path: '/admin/locales/:code/edit',
    component: AdminLocalesForm
  }, {
    name: 'users.profile',
    path: '/users/profile',
    component: UsersProfile
  }, {
    name: 'users.change-password',
    path: '/users/change-password',
    component: UsersChangePassword
  }, {
    name: 'users.logs',
    path: '/users/logs',
    component: UsersLogs
  }, {
    name: 'users.sign-in',
    path: '/users/sign-in',
    component: UsersSignIn
  }, {
    name: 'users.sign-up',
    path: '/users/sign-up',
    component: UsersSignUp
  }, {
    name: 'users.confirm',
    path: '/users/confirm',
    component: UsersConfirm
  }, {
    name: 'users.confirm.token',
    path: '/users/confirm/:token',
    component: UsersConfirmToken
  }, {
    name: 'users.unlock.token',
    path: '/users/unlock/:token',
    component: UsersUnlockToken
  }, {
    name: 'users.unlock',
    path: '/users/unlock',
    component: UsersUnlock
  }, {
    name: 'users.forgot-password',
    path: '/users/forgot-password',
    component: UsersForgotPassword
  }, {
    name: 'users.reset-password.token',
    path: '/users/reset-password/:token',
    component: UsersResetPassword
  }, {
    name: 'leave-words.new',
    path: '/leave-words/new',
    component: NewLeaveWord
  }
]
