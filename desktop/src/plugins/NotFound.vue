<template>
<exception :reason="$t('errors.not-found.title')" :button="{to:{name:'home'},label:$t('errors.not-found.button')}" />
</template>

<script>
export default {
  name: 'NotFound'
}
</script>
