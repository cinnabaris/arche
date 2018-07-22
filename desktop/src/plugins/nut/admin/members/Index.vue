<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <div style="float: right;">
    <el-button size="mini" type="primary" @click="()=>this.$router.push({name:'admin.members.new'})" class="el-icon-plus" />
  </div>
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column :label="$t('attributes.username')" width="320">
      <template slot-scope="scope">
        {{scope.row.nickName}}[{{scope.row.realName}}]({{$t(`attributes.gender-${scope.row.gender}`)}})
      </template>
    </el-table-column>
    <el-table-column :label="$t('attributes.contact')">
      <template slot-scope="scope">
        <el-card shadow="never">
          <div v-if="scope.row[it]" v-for="it in ['phone', 'email', 'address', 'line', 'wechat', 'skype', 'weibo', 'facebook']" :key="it">
            {{$t(`nut.attributes.member.${it}`)}}: {{scope.row[it]}}
          </div>
        </el-card>
      </template>
    </el-table-column>
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="120">
      <template slot-scope="scope">
        <el-button-group>
          <el-button size="mini" type="warning" @click="handleEdit(scope.row.id)" class="el-icon-edit"/>
          <el-button size="mini" type="danger" @click="handleDelete(scope.row.id, scope.row.nickName)" class="el-icon-delete"/>
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
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminMembersIndex',
  data() {
    return {
      role: {
        name: ADMIN,
        type: null,
        id: null
      },
      title: this.$t("nut.admin.members.index.title"),
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
        name: 'admin.members.edit',
        params: {
          id
        }
      })
    },
    handleDelete(id, name) {
      this.$confirm(this.$t('are-you-sure.delete', {
        id: name
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          removeMember(id: $id) {
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
        listMember{
          id, nickName, realName, gender, birthday, phone, email, address, line, wechat, skype, weibo, facebook
        }
      }`, {}).then((rst) => {
        this.items = rst.listMember
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
