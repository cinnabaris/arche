<template>
<el-container v-if="allow">
  <el-header/>
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

<exception v-else :reason="$t('errors.forbidden.title')" :button="{to:{name:'users.sign-in'},label:$t('errors.forbidden.button')}" />
</template>

<script>
import Footer from '../Footer'


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
  },
  computed: {
    allow() {
      return check(this.$store.state.currentUser, this.role)
    }
  }
}
</script>
