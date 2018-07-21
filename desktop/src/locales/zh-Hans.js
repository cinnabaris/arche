export default {
  languages: {
    "en-US": "English",
    "zh-Hans": "简体中文",
    "zh-Hant": "繁體中文"
  },
  flashes: {
    success: '操作成功',
    info: '提示信息',
    error: '出错了'
  },
  'are-you-sure': {
    delete: '您确定要删除{id}么？'
  },
  buttons: {
    submit: "提交",
    reset: '重置',
    ok: '确定',
    cancel: '取消',
    operator: '操作',
    return: '返回',
    enable: '启用',
    disable: '禁用'
  },
  attributes: {
    email: '电子邮箱',
    username: '用户名',
    password: '密码',
    'password-confirmation': '密码确认',
    'current-password': '当前密码',
    'new-password': '新密码',
    body: '正文',
    content: '内容',
    id: 'ID',
    host: '主机',
    port: '端口',
    ip: 'IP地址',
    name: '名称',
    title: '标题',
    url: '地址',
    subhead: '副标题',
    keywords: '关键字',
    description: '说明信息',
    copyright: '版权信息',
    loc: '位置',
    x: '横轴',
    y: '纵轴',
    logo: '图标',
    position: '排序',
    'created-at': '创建时间',
    'updated-at': '修改时间',
    'date-range': '日期区间',
    to: '至',
    nbf: '起始',
    exp: '截止',
    lang: '语言'
  },
  validations: {
    email: '不是正确的邮件地址',
    required: '不能为空',
    password: '密码长度应该在6位和32位之间',
    'password-confirmation': '两次密码输入不一致',
    'date-range': '请选择日期区间'
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
  header: {
    'sign-out': {
      confirm: '你确定要注销登录么？'
    }
  },
  nut: {
    install: {
      title: '初始化安装'
    },
    admin: {
      dashboard: {
        title: '站点参数'
      },
      site: {
        status: {
          title: '当前状态',
          os: '操作系统',
          app: '应用'
        },
        info: {
          title: '基本信息'
        },
        author: {
          title: '作者信息'
        },
        seo: {
          title: '搜索引擎'
        },
        smtp: {
          title: '邮件发送'
        }
      },
      'friend-links': {
        index: {
          title: '友情链接'
        }
      },
      'leave-words': {
        index: {
          title: '留言板'
        }
      },
      links: {
        index: {
          title: '导航链接'
        }
      },
      cards: {
        index: {
          title: '卡片区块'
        }
      },
      locales: {
        index: {
          title: '国际化'
        }
      },
      users: {
        index: {
          title: '用户',
          lock: '你确定要锁定用户{name}么？',
          prolicy: '权限'
        },
        policy: {
          title: '权限设置',
          admin: '超级管理员',
          caring: '会员关怀模块',
          forum: '论坛模块',
          donate: '捐赠模块'
        }
      }
    },
    users: {
      dashboard: {
        title: '账户信息'
      },
      profile: {
        title: '个人信息'
      },
      'change-password': {
        title: '修改密码'
      },
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
    },
    attributes: {
      user: {
        logo: '头像',
        'last-sign-in': '上次登录',
        'current-sign-in': '本次登录',
        'info': '信息'
      },
      locale: {
        code: '代码'
      },
      seo: {
        google: '谷歌网站ID',
        baidu: '百度网站ID'
      },
      smtp: {
        user: '发送者'
      },
      card: {
        action: '动作'
      }
    }
  },
  forum: {
    dashboard: {
      title: '交流论坛'
    },
    attributes: {
      topic: {
        tags: '标签'
      }
    },
    topics: {
      index: {
        title: '话题'
      }
    },
    posts: {
      index: {
        title: '讨论'
      }
    },
    tags: {
      index: {
        title: '标签'
      }
    }
  },
  cbeta: {
    dashboard: {
      title: '电子书'
    }
  },
  pos: {
    dashboard: {
      title: 'POS系统'
    }
  },
  shop: {
    dashboard: {
      title: '网上商城'
    }
  },
  caring: {
    dashboard: {
      title: '会员关怀'
    }
  },
  library: {
    dashboard: {
      title: '图书馆'
    }
  },
  hotel: {
    dashboard: {
      title: '住宿管理'
    }
  },
  todo: {
    dashboard: {
      title: '任务管理'
    }
  },
  donate: {
    dashboard: {
      title: '捐赠项目'
    }
  },
  ops: {
    vpn: {
      dashboard: {
        title: 'VPN系统'
      }
    },
    email: {
      dashboard: {
        title: 'EMAIL系统'
      }
    }
  }
}
