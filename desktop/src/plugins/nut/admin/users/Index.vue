<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <el-pagination @current-change="setPage" :page-size="size" layout="total, prev, pager, next" :total="items.length" />
  <el-table :data="table" border>
    <el-table-column :label="$t('nut.attributes.user.info')">
      <template slot-scope="scope">
        {{scope.row.name}}&lt;{{scope.row.email}}&gt;[{{scope.row.signInCount}}]
      </template>
    </el-table-column>
    <el-table-column :label="$t('nut.attributes.user.last-sign-in')">
      <template slot-scope="scope">
        {{scope.row.lastSignInAt}}[{{scope.row.lastSignInIp}}]
      </template>
    </el-table-column>
    <el-table-column :label="$t('nut.attributes.user.current-sign-in')">
      <template slot-scope="scope">
        {{scope.row.currentSignInAt}}[{{scope.row.currentSignInIp}}]
      </template>
    </el-table-column>
    <el-table-column fixed="right" :label="$t('buttons.operator')" width="120">
      <template slot-scope="scope">
        <el-button-group>
          <el-button size="mini" type="warning" @click="handlePolicy(scope.row.id)" class="el-icon-menu"/>
          <el-button size="mini" type="danger" @click="handleLock(scope.row.id, scope.row.name)" class="el-icon-bell"/>
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
  name: 'AdminUsersIndex',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.users.index.title"),
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
    handlePolicy(id) {
      this.$router.push({
        name: 'admin.users.policy',
        params: {
          id
        }
      })
    },
    handleLock(id, name) {
      this.$confirm(this.$t('nut.admin.users.index.lock', {
        name
      }), this.$t('flashes.info'), {
        confirmButtonText: this.$t('buttons.ok'),
        cancelButtonText: this.$t('buttons.cancel'),
        type: 'warning',
        center: true
      }).then(() => {
        client().request(`mutation form($id: String!){
          lockUser(id: $id) {
            createdAt
          }
        }`, {
          id
        }).then(() => {
          this.$message({
            type: 'success',
            message: this.$t("flashes.success")
          })
        }).catch(failed)
      }).catch(() => {})

    },
    init() {
      client().request(`query list{
        listUser{
          id, name, email, lastSignInAt, lastSignInIp, currentSignInAt, currentSignInIp, signInCount
        }
      }`, {}).then((rst) => {
        this.items = rst.listUser
      }).catch(failed)
    },
    setPage(p) {
      this.page = p
    }
  }
}
</script>
