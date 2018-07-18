<template>
<el-menu @select="handleSelect">
  <el-menu-item :key="it.to" v-for="it in items" :index="it.to">
    <icon-font :name="it.icon" />&nbsp;
    <span slot="title">{{$t(it.label)}}</span>
  </el-menu-item>
</el-menu>
</template>

<script>
export default {
  name: 'UsersSharedLinks',
  data() {
    return {
      items: [{
        icon: 'login',
        to: 'users.sign-in',
        label: 'nut.users.sign-in.title'
      }, {
        icon: 'adduser',
        to: 'users.sign-up',
        label: 'nut.users.sign-up.title'
      }, {
        icon: 'key',
        to: 'users.forgot-password',
        label: 'nut.users.forgot-password.title'
      }, {
        icon: 'checkcircleo',
        to: 'users.confirm',
        label: 'nut.users.confirm.title'
      }, {
        icon: 'unlock',
        to: 'users.unlock',
        label: 'nut.users.unlock.title'
      }, {
        icon: 'message1',
        to: 'leave-words.new',
        label: 'nut.leave-words.new.title'
      }]
    }
  },
  methods: {
    handleSelect(key) {
      this.$router.push({
        name: key
      })
    }
  }
}
</script>
