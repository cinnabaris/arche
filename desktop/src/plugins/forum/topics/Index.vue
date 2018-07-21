<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <div style="float: right;">
    <el-button size="mini" type="primary" @click="()=>this.$router.push({name:'forum.topics.new'})" class="el-icon-plus" />
  </div>
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="id" :label="$t('attributes.id')" width="60" />
    <el-table-column :label="$t('attributes.lang')" width="80">
      <template slot-scope="scope">
        {{$t(`languages.${scope.row.lang}`)}}
      </template>
    </el-table-column>
    <el-table-column :label="$t('attributes.updated-at')" width="240">
      <template slot-scope="scope">
        <timestamp :value="scope.row.updatedAt" />
      </template>
    </el-table-column>
    <el-table-column :label="$t('attributes.title')">
      <template slot-scope="scope">
        <a :href="`/forum/topics/${scope.row.id}`" target="_blank">{{scope.row.title}}</a>
      </template>
    </el-table-column>
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="160">
      <template slot-scope="scope">
        <el-button-group>
          <el-button size="mini" type="info" @click="handlePost(scope.row.id)" class="el-icon-document"/>
          <el-button v-if="scope.row.editable" size="mini" type="warning" @click="handleEdit(scope.row.id)" class="el-icon-edit"/>
          <el-button v-if="scope.row.editable" size="mini" type="danger" @click="handleDelete(scope.row.id, scope.row.title)" class="el-icon-delete"/>
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
  name: 'ForumTopicsIndex',
  data() {
    return {
      role: FORUM,
      title: this.$t("forum.topics.index.title"),
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
    handlePost(id) {
      this.$router.push({
        name: 'forum.posts.new',
        params: {
          topic: id
        }
      })
    },
    handleEdit(id) {
      this.$router.push({
        name: 'forum.topics.edit',
        params: {
          id
        }
      })
    },
    handleDelete(id, title) {
      this.$confirm(this.$t('are-you-sure.delete', {
        id: title
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          removeForumTopic(id: $id) {
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
        listForumTopic{
          id, title, lang, editable, updatedAt
        }
      }`, {}).then((rst) => {
        this.items = rst.listForumTopic
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
