<template>
<layout-application>
  sign in
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
