<template>
<div />
</template>

<script>
export default {
  name: 'DocumentTitle',
  created() {
    document.title = this.title + '|' + this.$store.state.siteInfo.subhead + '|' + this.$store.state.siteInfo.title
  },
  props: {
    title: String
  }
}
</script>
