<template>
<dashboard-layout :title="title" :role="null">
  <div>
    change-password
  </div>
</dashboard-layout>
</template>

<script>
export default {
  name: 'UsersChangePassword',
  data() {
    return {
      title: this.$t("nut.users.change-password.title"),
      items: []
    }
  }
}
</script>
