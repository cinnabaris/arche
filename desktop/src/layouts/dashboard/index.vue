<template>
<el-container style="min-height: 100%;" v-if="allow">
  <el-aside>
    <sider-bar/>
  </el-aside>
  <el-container>
    <el-header>
      <header-bar />
    </el-header>
    <el-main>
      <el-row>
        <el-col :md="{offset:8, span:8}" :sm="{span: 24}">
          <document-title :title="title" />
          <slot/>
        </el-col>
      </el-row>
    </el-main>
    <layout-footer/>
  </el-container>
</el-container>

<exception v-else :reason="$t('errors.forbidden.title')" :button="{to:{name:'users.sign-in'},label:$t('errors.forbidden.button')}" />
</template>

<script>
import Footer from '../Footer'
import Sider from './Sider'
import Header from './Header'

import {
  getToken
} from '@/utils'

import {
  check
} from '@/authorized'

export default {
  name: 'DashboardLayout',
  props: {
    role: String,
    title: String,
  },
  created() {
    if (!this.$store.state.currentUser) {
      var token = getToken()
      if (token) {
        this.$store.commit('signIn', token)
      }
    }
  },
  components: {
    'layout-footer': Footer,
    'sider-bar': Sider,
    'header-bar': Header,
  },
  computed: {
    allow() {
      return check(this.$store.state.currentUser, this.role)
    }
  }
}
</script>
