<template>
<email-form :action="action" :query="query" />
</template>

<script>
import EmailForm from './EmailForm'

export default {
  name: 'UsersConfirm',
  components: {
    'email-form': EmailForm
  },
  data() {
    return {
      action: 'confirm',
      query: `query form($email: String!){
        confirmUser(email: $email) {
          createdAt
        }
      }`
    }
  }
}
</script>
