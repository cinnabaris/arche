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
  name: 'layout-application',
  components: {
    Header,
    Footer
  }
}
</script>
