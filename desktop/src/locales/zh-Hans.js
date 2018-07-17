export default {
  languages: {
    "en-US": "English",
    "zh-Hans": "简体中文",
    "zh-Hant": "繁體中文"
  },
  flashes: {
    success: '操作成功'
  },
  buttons: {
    submit: "提交",
    reset: '重置'
  },
  attributes: {
    email: '电子邮箱',
    username: '用户名',
    password: '密码',
    'password-confirmation': '密码确认',
    body: '正文',
    content: '内容'
  },
  validations: {
    email: '不是正确的邮件地址',
    required: '不能为空',
    password: '密码长度应该在6位和32位之间',
    'password-confirmation': '两次密码输入不一致'
  },
  errors: {
    forbidden: {
      title: '禁止的访问',
      button: '返回登录'
    },
    'not-found': {
      title: '页面不存在',
      button: '返回主页'
    }
  },
  nut: {
    install: {
      title: '初始化安装'
    },
    users: {
      'logs': {
        title: '日志'
      },
      'sign-in': {
        title: '现有用户登录'
      },
      'sign-up': {
        title: '新用户注册'
      },
      'confirm': {
        title: '没有收到激活邮件？',
        success: '你将会收到一封激活邮件',
        token: {
          success: '激活成功，请继续登录。'
        }
      },
      'unlock': {
        title: '没有收到解锁邮件？',
        success: '你将会收到一封解锁邮件',
        token: {
          success: '解锁成功，请继续登录。'
        }
      },
      'forgot-password': {
        title: '忘记密码？',
        success: '您将会收到一封重置密码邮件。'
      },
      'reset-password': {
        title: '重置密码',
        success: '密码重置成功，请继续登录。'
      }
    },
    'leave-words': {
      new: {
        title: '给我们留言'
      }
    }
  }
}
