<template>
<div>
  <dashboard-header/>
  dashboard layout
  <slot/>
  <dashboard-footer/>
</div>
</template>

<script>
import Header from './Header'
import Footer from './Footer'

export default {
  name: 'layout-dashboard',
  components: {
    'dashboard-header': Header,
    'dashboard-footer': Footer
  },
  data () {
    return {}
  }
}
</script>

<style scoped>
@import './index.css';
</style>
