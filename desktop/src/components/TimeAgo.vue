<template>
<span>{{date}}</span>
</template>

<script>
import moment from 'moment'

export default {
  name: 'TimeAgo',
  props: {
    value: String
  },
  computed: {
    date() {
      return moment(this.value).fromNow()
    }
  }
}
</script>
