<template>
<div v-loading="loading" />
</template>

<script>
export default {
  name: 'Home',
  created() {
    this.$router.push({
      name: this.$store.state.currentUser ? 'users.logs' : 'users.sign-in'
    })
  },
  data() {
    return {
      loading: true
    }
  }
}
</script>
