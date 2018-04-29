<template>
<div>
  <router-link :to="{name:'users.sign-in'}">Sign in</router-link>
  <router-link :to="{name:'users.sign-up'}">Sign up</router-link>
</div>
</template>

<script>
export default {
  name: 'users-shared-links',
  data () {
    return {}
  }
}
</script>
