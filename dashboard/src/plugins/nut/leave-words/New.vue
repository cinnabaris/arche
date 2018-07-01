<template>
<application-layout :title="$t('leave-words.new.title')" :on-submit="onSubmit">
  <v-textarea prepend-icon="feedback" :label="$t('attributes.content')" />
</application-layout>
</template>


<script>
export default {
  name: 'NewLeaveWord',
  methods: {
    onSubmit: function(event) {
      event.preventDefault()
      console.log('leave words new')
    }
  }
}
</script>
