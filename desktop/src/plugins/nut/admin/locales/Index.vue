<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <div style="float: right;">
    <el-button size="mini" type="primary" @click="()=>this.$router.push({name:'admin.locales.new'})" class="el-icon-plus" />
  </div>
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column prop="code" :label="$t('nut.attributes.locale.code')" width="280" />
    <el-table-column prop="message" :label="$t('attributes.content')" />
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="120">
      <template slot-scope="scope">
        <el-button-group>
          <el-button size="mini" type="warning" @click="handleEdit(scope.row.code)" class="el-icon-edit"/>
          <el-button size="mini" type="danger" @click="handleDelete(scope.row.id, scope.row.code)" class="el-icon-delete"/>
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

export default {
  name: 'AdminLocalesIndex',
  data() {
    return {
      title: this.$t("nut.admin.locales.index.title"),
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
    handleEdit(code) {
      this.$router.push({
        name: 'admin.locales.edit',
        params: {
          code
        }
      })
    },
    handleDelete(id, code) {
      this.$confirm(this.$t('are-you-sure.delete', {
        id: code
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          removeLocale(id: $id) {
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
      client().request(`query locales($lang: String!){
        listLocaleByLang(lang: $lang) {
          id, code, message
        }
      }`, {
        lang: this.$i18n.locale
      }).then((rst) => {
        this.items = rst.listLocaleByLang
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
