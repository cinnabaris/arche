<template>
<dashboard-layout :title="title" :role="null">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.current-password')" prop="currentPassword">
          <el-input type="password" v-model="form.currentPassword" clearable auto-complete="off" />
        </el-form-item>
        <el-form-item :label="$t('attributes.new-password')" prop="newPassword">
          <el-input type="password" v-model="form.newPassword" clearable auto-complete="off" />
        </el-form-item>
        <el-form-item :label="$t('attributes.password-confirmation')" prop="passwordConfirmation">
          <el-input type="password" v-model="form.passwordConfirmation" clearable auto-complete="off" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm('form')">{{$t('buttons.submit')}}</el-button>
          <el-button @click="resetForm('form')">{{$t('buttons.reset')}}</el-button>
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
  name: 'UsersChangePassword',
  data() {
    var validatePasswords = (rule, value, callback) => {
      if (value === '' || value !== this.form.newPassword) {
        callback(new Error(this.$t('validations.password-confirmation')));
      } else {
        callback();
      }
    };
    return {
      title: this.$t("nut.users.change-password.title"),
      form: {
        currentPassword: '',
        newPassword: '',
        passwordConfirmation: ''
      },
      rules: {
        currentPassword: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        newPassword: [{
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
          client().request(`mutation form($currentPassword: String!, $newPassword: String!){
            changeUserPassword(currentPassword: $currentPassword, newPassword: $newPassword) {
              createdAt
            }
          }`, {
            currentPassword: this.form.currentPassword,
            newPassword: this.form.newPassword
          }).then(() => {
            this.$message({
              type: 'success',
              message: this.$t("flashes.success")
            })
            this.$refs[formName].resetFields();
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
