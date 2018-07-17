<template>
<dashboard-layout :title="title" :role="null">
  <div>
    logs
  </div>
</dashboard-layout>
</template>

<script>
export default {
  name: 'UsersLogs',
  data() {
    return {
      title: this.$t("nut.users.logs.title"),
      items: []
    }
  }
}
</script>
