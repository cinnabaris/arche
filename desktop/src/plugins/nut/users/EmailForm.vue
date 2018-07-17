<template>
<application-layout>
  <document-title :title="title" />
  <el-card :header="title">
    <el-form :rules="rules" ref="form" :model="form" label-width="80px">
      <el-form-item :label="$t('attributes.email')" prop="email">
        <el-input v-model="form.email" clearable required/>
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
  name: 'UsersEmailForm',
  components: {
    'shared-links': SharedLinks
  },
  props: {
    action: String,
    query: String,
  },
  data() {
    return {
      title: this.$t(`nut.users.${this.action}.title`),
      form: {
        email: ''
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
      }
    }
  },
  methods: {
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          client().request(this.query, {
            email: this.form.email
          }).then(() => {
            this.$message({
              type: 'success',
              message: this.$t(`nut.users.${this.action}.success`)
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
