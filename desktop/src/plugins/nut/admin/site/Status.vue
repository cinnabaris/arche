<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="name" :label="$t('attributes.name')" width="180" />
    <el-table-column prop="value" :label="$t('attributes.content')" />
  </el-table>
</dashboard-layout>
</template>

<script>
import {
  client,
  failed
} from '@/request'
import lodash from 'lodash'
import {
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminSiteStatus',
  data() {
    return {
      role: ADMIN,
      title: this.$t("nut.admin.site.status.title"),
      size: 12,
      page: 1,
      items: []
    }
  },
  computed: {
    table() {
      var offset = (this.page - 1) * this.size;
      return lodash(this.items).slice(offset).take(this.size).value()
    }
  },
  methods: {
    init() {
      client().request(`query list{
        getSiteStatus {
          name,
          value
        }
      }`, {}).then((rst) => {
        this.items = rst.getSiteStatus
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
