<template>
<span>{{date}}</span>
</template>

<script>
import moment from 'moment-timezone'

export default {
  name: 'Timestamp',
  props: {
    value: String
  },
  computed: {
    date() {
      return moment(this.value).tz(moment.tz.guess()).format('llll zz')
    }
  }
}
</script>
