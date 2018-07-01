<template>
<application-layout :title="$t('users.confirm.title')" :on-submit="onSubmit">
  <v-text-field prepend-icon="person" :label="$t('attributes.email')" type="text" />
</application-layout>
</template>


<script>
export default {
  name: 'UsersConfirm',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('confirm')
    }
  }
}
</script>
