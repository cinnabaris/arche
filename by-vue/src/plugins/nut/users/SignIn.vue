<template>
<application-layout :title="$t('users.sign-in.title')" :on-submit="onSubmit">
  <v-text-field prepend-icon="person" :label="$t('attributes.email')" type="text" />
  <v-text-field prepend-icon="lock" :label="$t('attributes.password')" type="password" />
</application-layout>
</template>


<script>
export default {
  name: 'UsersSignIn',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('sign in')
    }
  }
}
</script>
