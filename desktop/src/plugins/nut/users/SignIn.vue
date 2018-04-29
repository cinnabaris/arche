<template>
<layout-application>
  sign in <br/>{{ $t("nut.users.sign-in.title") }}
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
