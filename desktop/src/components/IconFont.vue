<template>
<i :class="`icon anticon icon-${name}`" />
</template>

<script>
export default {
  name: 'IconFont',
  props: {
    name: String
  }
}
</script>
