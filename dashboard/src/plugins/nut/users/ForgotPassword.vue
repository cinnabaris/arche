<template>
<application-layout :title="$t('users.forgot-password.title')" :on-submit="onSubmit">
  <v-text-field prepend-icon="person" :label="$t('attributes.email')" type="text" />
</application-layout>
</template>


<script>
export default {
  name: 'UsersForgotPassword',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('forgot-password')
    }
  }
}
</script>
