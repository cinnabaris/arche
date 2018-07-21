<template>
<el-menu mode="horizontal" @select="handleSelect">
  <el-menu-item index="sider-bar">
    <icon-font :name="$store.state.siderBar.show ? 'menu-unfold' : 'menu-fold'" />
  </el-menu-item>
  <el-menu-item index="go-home">
    <icon-font name='home' />
  </el-menu-item>
  <el-menu-item style="float: right;" index="sign-out">
    <icon-font name='logout' />
  </el-menu-item>
</el-menu>
</template>

<script>
import {
  client,
  failed
} from '@/request'

export default {
  name: 'Header',
  methods: {
    handleSelect(key) {
      switch (key) {
        case 'sider-bar':
          this.$store.commit('toggleSiderBar')
          return
        case 'sign-out':
          this.$confirm(this.$t('header.sign-out.confirm'), this.$t('flashes.info'), {
            confirmButtonText: this.$t('buttons.ok'),
            cancelButtonText: this.$t('buttons.cancel'),
            type: 'warning',
            center: true
          }).then(() => {
            client().request(`mutation form{
              signOutUser {
                createdAt
              }
            }`, {}).then(() => {
              this.$message({
                type: 'success',
                message: this.$t('flashes.success')
              });
              this.$store.commit('signOut')
              this.$router.push({
                name: 'users.sign-in'
              })
            }).catch(failed)
          }).catch(() => {})
          return
        case 'go-home':
          var win = window.open("/", '_blank')
          win.focus()
          return
        default:
          // eslint-disable-next-line
          console.log(key)
      }
    }
  }
}
</script>
