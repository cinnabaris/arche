<template>
<email-form :action="action" :query="query" />
</template>

<script>
import EmailForm from './EmailForm'

export default {
  name: 'UsersForgotPassword',
  components: {
    'email-form': EmailForm
  },
  data() {
    return {
      action: 'forgot-password',
      query: `query form($email: String!){
        forgotUserPassword(email: $email) {
          createdAt
        }
      }`
    }
  }
}
</script>
