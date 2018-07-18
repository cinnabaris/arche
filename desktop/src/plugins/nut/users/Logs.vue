<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="createdAt" :label="$t('attributes.createdAt')" width="240">
      <template slot-scope="scope">
        <timestamp :value="scope.row.createdAt"/>
      </template>
    </el-table-column>
    <el-table-column prop="ip" :label="$t('attributes.ip')" width="180" />
    <el-table-column prop="message" :label="$t('attributes.content')" />
  </el-table>
</dashboard-layout>
</template>

<script>
import {
  client,
  failed
} from '@/request'
import lodash from 'lodash'

export default {
  name: 'UsersLogs',
  data() {
    return {
      title: this.$t("nut.users.logs.title"),
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
      client().request(`query logs{
        listUserLog {
          id,
          ip,
          message,
          createdAt
        }
      }`, {}).then((rst) => {
        this.items = rst.listUserLog
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
