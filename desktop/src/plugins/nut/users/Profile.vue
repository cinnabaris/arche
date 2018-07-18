<template>
<dashboard-layout :title="title" :role="null">
  <div>
    profile
  </div>
</dashboard-layout>
</template>

<script>
export default {
  name: 'UsersProfile',
  data() {
    return {
      title: this.$t("nut.users.profile.title"),
      items: []
    }
  }
}
</script>
