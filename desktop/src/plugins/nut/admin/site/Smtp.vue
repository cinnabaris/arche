<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.host')" prop="host">
          <el-input v-model="form.host" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.port')">
          <el-select v-model="form.port">
            <el-option v-for="it in [25, 465, 587]" :key="it" :label="it" :value="it" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.smtp.user')" prop="user">
          <el-input v-model="form.user" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.password')" prop="password">
          <el-input type="password" v-model="form.password" clearable auto-complete="off" />
        </el-form-item>
        <el-form-item :label="$t('attributes.password-confirmation')" prop="passwordConfirmation">
          <el-input type="password" v-model="form.passwordConfirmation" clearable auto-complete="off" />
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

export default {
  name: 'AdminSiteAuthor',
  data() {
    var validatePasswords = (rule, value, callback) => {
      if (value === '' || value !== this.form.password) {
        callback(new Error(this.$t('validations.password-confirmation')));
      } else {
        callback();
      }
    };

    return {
      title: this.$t("nut.admin.site.smtp.title"),
      form: {
        host: '',
        port: 25,
        user: '',
        password: '',
        passwordConfirmation: ''
      },
      rules: {
        host: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        user: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }, {
          type: 'email',
          message: this.$t('validations.email'),
          trigger: ['blur', 'change']
        }],
        password: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }, {
          min: 6,
          max: 32,
          message: this.$t('validations.password'),
          trigger: ['blur', 'change']
        }],
        passwordConfirmation: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }, {
          validator: validatePasswords,
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
            `mutation form($host: String!, $port: Int!, $user: String!, $password: String!){
            updateSiteSmtp(host: $host, port: $port, user: $user, password: $password) {
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
        getSiteSmtp{
          host, port, user, password
        }
      }`, {}).then((rst) => {
        this.form = Object.assign({
          passwordConfirmation: ''
        }, rst.getSiteSmtp)
      }).catch(() => {})
    }
  }
}
</script>
