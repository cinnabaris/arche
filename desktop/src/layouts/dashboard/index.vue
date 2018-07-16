<template>
<div>
  <div> dashboard header </div>
  <slot/>
  <div> dashboard footer </div>
</div>
</template>

<script>
export default {
  name: 'DashboardLayout',
  props: {}
}
</script>
