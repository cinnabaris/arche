<template>
<application-layout :title="title">
  <el-card :header="title">
    <el-form :rules="rules" ref="form" :model="form" label-width="80px">
      <el-form-item :label="$t('attributes.email')" prop="email">
        <el-input v-model="form.email" clearable required/>
      </el-form-item>
      <el-form-item :label="$t('attributes.password')" prop="password">
        <el-input type="password" v-model="form.password" clearable auto-complete="off" />
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
import {
  setToken
} from '@/utils'

export default {
  name: 'UsersSignIn',
  components: {
    'shared-links': SharedLinks
  },
  data() {
    return {
      title: this.$t('nut.users.sign-in.title'),
      form: {
        name: '',
        password: ''
      },
      rules: {
        email: [{
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
        }]
      }
    }
  },
  methods: {
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          client().request(`mutation form($email: String!, $password: String!){
            signInUserByEmail(email: $email, password: $password) {
              token
            }
          }`, {
            email: this.form.email,
            password: this.form.password
          }).then((rst) => {
            this.$message({
              type: 'success',
              message: this.$t("flashes.success")
            })
            var token = rst.signInUserByEmail.token
            setToken(token)
            this.$store.commit('signIn', token)
            this.$router.push({
              name: 'users.logs'
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
