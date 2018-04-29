<template>
  <div>
    <router-link :to="{ name: 'users.sign-in'}">{{ $t("nut.users.sign-in.title") }}</router-link>
    <router-link :to="{ name: 'users.sign-up'}">{{ $t("nut.users.sign-up.title") }}</router-link>
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
