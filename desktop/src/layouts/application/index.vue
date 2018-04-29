<template>
<div>
  <application-header/>
  application layout
  <slot/>
  <application-footer/>
</div>
</template>

<script>
import Header from './Header'
import Footer from './Footer'

export default {
  name: 'layout-application',
  components: {
    'application-header': Header,
    'application-footer': Footer
  },
  data () {
    return {}
  }
}
</script>
