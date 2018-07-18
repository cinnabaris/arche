<template>
<span>{{date}}</span>
</template>

<script>
import moment from 'moment-timezone'

const TZ = moment.tz.guess()

export default {
  name: 'Timestamp',
  props: {
    value: String
  },
  computed: {
    date() {
      return moment(this.value).tz(TZ).format('llll zz')
    }
  }
}
</script>
