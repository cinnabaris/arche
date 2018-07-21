<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="160px">
        <el-form-item :key="it" v-for="it in roles" :label="$t(`nut.admin.users.policy.${it}`)">
          <el-switch v-model="form[it]" />
        </el-form-item>
        <el-form-item :label="$t('attributes.date-range')">
          <el-date-picker value-format="yyyy-MM-dd" v-model="form.dates" type="daterange" :range-separator="$t('attributes.to')" :start-placeholder="$t('attributes.nbf')" :end-placeholder="$t('attributes.exp')" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm('form')">{{$t('buttons.submit')}}</el-button>
          <el-button @click="go_back">{{$t('buttons.return')}}</el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </col-form>
</dashboard-layout>
</template>

<script>
import {
  client,
  failed
} from '@/request'
import {
  ADMIN,
  MANAGER,
} from '@/authorized'

export default {
  name: 'AdminUsersPolicy',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.users.policy.title"),
      form: {
        'r-admin': false,
        't-forum': false,
        't-caring': false,
        't-donate': false,
        dates: []
      },
      roles: [
        'r-admin', 't-forum', 't-caring', 't-donate'
      ],
      rules: {}
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'admin.users.index'
      })
    },
    submitForm() {
      var policies = JSON.stringify(this.roles.reduce((ar, it) => {
        if (this.form[it]) {
          if (it.startsWith('r-')) {
            ar.push({
              roleName: it.substring(2),
            })
          } else if (it.startsWith('t-')) {
            ar.push({
              roleName: MANAGER,
              resourceType: it.substring(2),
            })
          }
        }
        return ar
      }, []))

      var id = this.$route.params.id
      if (this.form.dates.length < 2) {
        this.$notify.error({
          title: this.$t('flashes.error'),
          message: this.$t('validations.date-range')
        })
        return
      }
      client().request(
        `mutation form($user: String!, $policies: String!, $nbf: String!, $exp: String!){
            updateUserPolicy(user: $user, policies: $policies, nbf: $nbf, exp: $exp) {
              createdAt
            }
          }`, {
          user: id,
          nbf: this.form.dates[0],
          exp: this.form.dates[1],
          policies,
        }).then(() => {
        this.$message({
          type: 'success',
          message: this.$t("flashes.success")
        })
        this.go_back()
      }).catch(failed)
    },
    init() {
      var id = this.$route.params.id
      client().request(`query info($id: String!){
          listManagerByUser(id: $id) {
            roleName, resourceType, resourceId
          }
        }`, {
        id
      }).then((rst) => {
        rst.listManagerByUser.forEach((it) => {
          if (!it.resourceId) {
            if (it.resourceType) {
              if (it.roleName == MANAGER) {
                // module manager
                this.form[`t-${it.resourceType}`] = true
              }
            } else {
              // all module
              this.form[`r-${it.roleName}`] = true
            }
          }
        })
      }).catch(failed)
    }

  }
}
</script>
