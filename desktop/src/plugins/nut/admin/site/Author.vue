<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.username')" prop="name">
          <el-input v-model="form.name" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.email')" prop="email">
          <el-input v-model="form.email" clearable required/>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm('form')">{{$t('buttons.submit')}}</el-button>
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
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminSiteAuthor',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.site.author.title"),
      form: {
        name: '',
        email: ''
      },
      rules: {
        name: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        email: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }, {
          type: 'email',
          message: this.$t('validations.email'),
          trigger: ['blur', 'change']
        }]
      }
    }
  },
  methods: {
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          client().request(
            `mutation form($name: String!, $email: String!){
            updateSiteAuthor(name: $name, email: $email) {
              createdAt
            }
          }`,
            this.form).then(() => {
            this.$message({
              type: 'success',
              message: this.$t("flashes.success")
            })
          }).catch(failed)
        } else {
          return false;
        }
      });
    },
    init() {
      client().request(`query info{
        getSiteAuthor{
          name, email
        }
      }`, {}).then((rst) => {
        this.form = rst.getSiteAuthor
      }).catch(() => {})
    }
  }
}
</script>
