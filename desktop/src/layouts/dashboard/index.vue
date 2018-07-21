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
      <document-title :title="title" />
      <el-row>
        <slot/>
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
import {
  client
} from '@/request'

export default {
  name: 'DashboardLayout',
  props: {
    role: Object,
    title: String,
    init: Function
  },
  data() {
    return {
      allow: false
    }
  },
  created() {
    if (!this.$store.state.currentUser.uid) {
      var token = getToken()
      if (token) {
        this.$store.commit('signIn', token)
      }
    }
    if (!this.$store.state.currentUser.policies) {
      client().request(`query info{
        listUserPolicy {
          roleName, resourceType, resourceId
        }
      }`, {}).then((rst) => {
        this.$store.commit('updatePolicies', rst.listUserPolicy)
        this.auth()
      }).catch(() => {})
    } else {
      this.auth()
    }
  },
  components: {
    'layout-footer': Footer,
    'sider-bar': Sider,
    'header-bar': Header,
  },

  methods: {
    auth() {
      if (check(this.$store.state.currentUser, this.role)) {
        if (this.init) {
          this.init()
        }
        this.allow = true
      }
    }
  }
}
</script>
