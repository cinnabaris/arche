<template>
<layout-application>
  sign in
   <b-alert show>Default Alert</b-alert>
  <shared-links/>
</layout-application>
</template>

<script>
import SharedLinks from './SharedLinks'

export default {
  name: 'users-sign-in',
  components: {
    'shared-links': SharedLinks
  },
  data () {
    return {}
  }
}
</script>
