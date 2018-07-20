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
import lodash from 'lodash'
import {
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminUsersPolicy',
  data() {
    return {
      role: ADMIN,
      title: this.$t("nut.admin.users.policy.title"),
      form: {
        admin: false,
        forum: false,
        caring: false,
        donate: false,
        dates: []
      },
      roles: [
        'admin', 'forum', 'caring', 'donate'
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
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
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
            policies: JSON.stringify(this.roles.map((it) => {
              return {
                name: it,
                enable: this.form[it]
              }
            })),
          }).then(() => {
          this.$message({
            type: 'success',
            message: this.$t("flashes.success")
          })
          this.go_back()
        }).catch(failed)

      });
    },
    init() {
      var id = this.$route.params.id
      client().request(`query info($id: String!){
          getUserPolicy(id: $id) {
            name
          }
        }`, {
        id
      }).then((rst) => {
        rst.getUserPolicy.forEach((it) => {
          this.form[it.name] = true
        })
      }).catch(failed)
    }

  }
}
</script>
