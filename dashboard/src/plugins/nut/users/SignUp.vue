<template>
<application-layout :title="$t('users.sign-up.title')" :on-submit="onSubmit">
  <v-text-field prepend-icon="person" label="Login" type="text" />
  <v-text-field prepend-icon="lock" label="Password" type="password" />
  <v-text-field prepend-icon="lock" label="Password" type="password" />
</application-layout>
</template>


<script>
export default {
  name: 'UsersSignUp',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('sign up')
    }
  }
}
</script>
