<template>
<application-layout :title="$t('users.unlock.title')" :on-submit="onSubmit">
  <v-text-field prepend-icon="person" :label="$t('attributes.email')" type="text" />
</application-layout>
</template>


<script>
export default {
  name: 'UsersUnlock',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('unlock')
    }
  }
}
</script>
