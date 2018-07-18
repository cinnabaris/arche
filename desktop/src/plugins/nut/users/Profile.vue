<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.email')" prop="email">
          <el-input v-model="form.email" disabled clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.username')" prop="name">
          <el-input v-model="form.name" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.user.logo')" prop="logo">
          <el-input v-model="form.logo" clearable required/>
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
  name: 'UsersProfile',
  data() {
    return {
      title: this.$t("nut.users.profile.title"),
      form: {
        email: '',
        name: '',
        logo: ''
      },
      rules: {
        name: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        logo: [{
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
          client().request(`mutation info($name: String!, $logo: String!){
            updateUserProfile(name: $name, logo: $logo) {
              createdAt
            }
          }`, {
            name: this.form.name,
            logo: this.form.logo
          }).then(() => {
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
        getUserProfile {
          name,
          logo,
          email
        }
      }`, {}).then((rst) => {
        var ifo = rst.getUserProfile
        this.form.name = ifo.name
        this.form.logo = ifo.logo
        this.form.email = ifo.email
      }).catch(failed)
    }
  }
}
</script>
