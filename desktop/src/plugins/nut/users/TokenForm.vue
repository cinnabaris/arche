<template>
<div v-loading="loading" />
</template>

<script>
import {
  client,
  failed
} from '@/request'

export default {
  name: 'UsersTokenForm',
  props: {
    action: String,
    query: String,
  },
  created() {
    client().request(this.query, {
      token: this.$route.params.token
    }).then(() => {
      this.$message({
        type: 'success',
        message: this.$t(`nut.users.${this.action}.token.success`)
      })
      this.$router.push({
        name: 'users.sign-in'
      })
    }).catch(failed)
  },
  data() {
    return {
      loading: true
    }
  }
}
</script>
