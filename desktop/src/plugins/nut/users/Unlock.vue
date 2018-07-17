<template>
<email-form :action="action" :query="query" />
</template>

<script>
import EmailForm from './EmailForm'

export default {
  name: 'UsersUnlock',
  components: {
    'email-form': EmailForm
  },
  data() {
    return {
      action: 'unlock',
      query: `query form($email: String!){
        unlockUser(email: $email) {
          createdAt
        }
      }`
    }
  }
}
</script>
