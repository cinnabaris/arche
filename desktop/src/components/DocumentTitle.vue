<template>
<span />
</template>

<script>
export default {
  name: 'DocumentTitle',
  created() {
    document.title = this.title
  },
  props: {
    title: String
  }
}
</script>
