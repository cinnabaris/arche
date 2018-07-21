<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <div style="float: right;">
    <el-button size="mini" type="primary" @click="()=>this.$router.push({name:'forum.tags.new'})" class="el-icon-plus" />
  </div>
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="id" :label="$t('attributes.id')" width="60" />
    <el-table-column prop="name" :label="$t('attributes.name')" width="120" />
    <el-table-column :label="$t('attributes.updated-at')">
      <template slot-scope="scope">
        <timestamp :value="scope.row.updatedAt" />
      </template>
    </el-table-column>
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="120">
      <template slot-scope="scope">
        <el-button-group>
          <el-button size="mini" type="warning" @click="handleEdit(scope.row.id)" class="el-icon-edit"/>
          <el-button size="mini" type="danger" @click="handleDelete(scope.row.id, scope.row.name)" class="el-icon-delete"/>
        </el-button-group>
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
  FORUM
} from '@/authorized'

export default {
  name: 'ForumTagsIndex',
  data() {
    return {
      role: FORUM,
      title: this.$t("forum.tags.index.title"),
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
    handleEdit(id) {
      this.$router.push({
        name: 'forum.tags.edit',
        params: {
          id
        }
      })
    },
    handleDelete(id, name) {
      this.$confirm(this.$t('are-you-sure.delete', {
        id: name,
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          removeForumTag(id: $id) {
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
        listForumTag{
          id, name, updatedAt
        }
      }`, {}).then((rst) => {
        this.items = rst.listForumTag
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
