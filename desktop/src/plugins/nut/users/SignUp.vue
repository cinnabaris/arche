<template>
<layout-application>
  sign up
  <shared-links/>
</layout-application>
</template>

<script>
import SharedLinks from './SharedLinks'

export default {
  name: 'users-sign-up',
  components: {
    'shared-links': SharedLinks
  },
  data () {
    return {}
  }
}
</script>
