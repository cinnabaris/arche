<template>
<token-form :action="action" :query="query" />
</template>

<script>
import TokenForm from './TokenForm'

export default {
  name: 'UsersConfirmToken',
  components: {
    'token-form': TokenForm
  },
  data() {
    return {
      action: 'confirm',
      query: `mutation form($token: String!){
        confirmUser(token: $token) {
          createdAt
        }
      }`
    }
  }
}
</script>
