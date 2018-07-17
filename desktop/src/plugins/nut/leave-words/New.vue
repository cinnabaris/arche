<template>
<application-layout :title="title">
  <el-card :header="title">
    <el-form :rules="rules" ref="form" :model="form" label-width="80px">
      <el-form-item :label="$t('attributes.content')" prop="body">
        <el-input type='textarea' v-model="form.body" clearable />
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
import SharedLinks from '../users/SharedLinks'
import {
  client,
  failed
} from '@/request'

export default {
  name: 'NewLeaveWord',
  components: {
    'shared-links': SharedLinks
  },
  data() {
    return {
      title: this.$t('nut.leave-words.new.title'),
      form: {
        body: ''
      },
      rules: {
        body: [{
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
          client().request(`mutation form($mediaType: String!, $body: String!){
            createLeaveWord(mediaType: $mediaType, body: $body) {
              createdAt
            }
          }`, {
            body: this.form.body,
            mediaType: 'text'
          }).then(() => {
            this.$message({
              type: 'success',
              message: this.$t("flashes.success")
            })
            this.form.body = ' '
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
