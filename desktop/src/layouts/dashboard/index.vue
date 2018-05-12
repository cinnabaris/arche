<template>
<div>
  <Header/>
  <slot/>
  <Footer/>
</div>
</template>

<script>
import Header from './header'
import Footer from './footer'

export default {
  name: 'layout-dashboard',
  components: {
    Header,
    Footer
  }
}
</script>
