<template>
<token-form :action="action" :query="query" />
</template>

<script>
import TokenForm from './TokenForm'

export default {
  name: 'UsersUnlockToken',
  components: {
    'token-form': TokenForm
  },
  data() {
    return {
      action: 'unlock',
      query: `mutation form($token: String!){
        unlockUser(token: $token) {
          createdAt
        }
      }`
    }
  }
}
</script>
