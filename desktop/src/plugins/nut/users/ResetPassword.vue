<template>
<application-layout>
  <document-title :title="title" />
  <el-card :header="title">
    <el-form :rules="rules" ref="form" :model="form" label-width="80px">
      <el-form-item :label="$t('attributes.password')" prop="password">
        <el-input type="password" v-model="form.password" clearable auto-complete="off" />
      </el-form-item>
      <el-form-item :label="$t('attributes.password-confirmation')" prop="passwordConfirmation">
        <el-input type="password" v-model="form.passwordConfirmation" clearable auto-complete="off" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitForm('form')">{{$t('buttons.submit')}}</el-button>
        <el-button @click="resetForm('form')">{{$t('buttons.reset')}}</el-button>
      </el-form-item>
    </el-form>
    <shared-links />
  </el-card>
</application-layout>
</template>

<script>
import SharedLinks from './SharedLinks'
import {
  client,
  failed
} from '@/request'

export default {
  name: 'UsersResetPassword',
  components: {
    'shared-links': SharedLinks
  },
  data() {
    var validatePasswords = (rule, value, callback) => {
      if (value === '' || value !== this.form.password) {
        callback(new Error(this.$t('validations.password-confirmation')));
      } else {
        callback();
      }
    };
    return {
      title: this.$t('nut.users.reset-password.title'),
      form: {
        password: '',
        passwordConfirmation: ''
      },
      rules: {
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
          client().request(`mutation form($token: String!, $password: String!){
            resetUserPassword(token: $token, password: $password) {
              createdAt
            }
          }`, {
            token: this.$route.params.token,
            password: this.form.password
          }).then(() => {
            this.$message({
              type: 'success',
              message: this.$t("nut.users.reset-password.success")
            })
            this.$router.push({
              name: 'users.sign-in'
            })
          }).catch(failed)
        } else {
          return false;
        }
      });
    },
    resetForm(formName) {
      this.$refs[formName].resetFields();
    }
  }
}
</script>
