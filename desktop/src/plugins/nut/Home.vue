<template>
<div v-loading="loading" />
</template>

<script>
import {
  getToken
} from '@/utils'

export default {
  name: 'Home',
  created() {
    this.$router.push({
      name: getToken() ? 'users.logs' : 'users.sign-in'
    })
  },
  data() {
    return {
      loading: true
    }
  }
}
</script>
