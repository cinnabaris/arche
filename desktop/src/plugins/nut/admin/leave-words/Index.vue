<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="id" :label="$t('attributes.id')" width="60" />
    <el-table-column prop="createdAt" :label="$t('attributes.created-at')" width="240">
      <template slot-scope="scope">
        <timestamp :value="scope.row.createdAt"/>
      </template>
    </el-table-column>
    <el-table-column :label="$t('attributes.content')">
      <template slot-scope="scope">
        <pre>{{scope.row.body}}</pre>
      </template>
    </el-table-column>
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="120">
      <template slot-scope="scope">
        <el-button size="mini" type="danger" @click="handleDelete(scope.row.id)" class="el-icon-delete"/>
      </template>
    </el-table-column>
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
  name: 'AdminLeaveWordsIndex',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.leave-words.index.title"),
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
    handleDelete(id) {
      this.$confirm(this.$t('are-you-sure.delete', {
        id
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          removeLeaveWord(id: $id) {
            createdAt
          }
        }`, {
          id
        }).then(() => {
          this.items = this.items.filter((it) => {
            return it.id !== id
          })
        }).catch(failed)
      }).catch(() => {})

    },
    init() {
      client().request(`query list{
        listLeaveWord{
          id, body, createdAt
        }
      }`, {}).then((rst) => {
        this.items = rst.listLeaveWord
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
