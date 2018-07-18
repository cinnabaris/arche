<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('nut.attributes.locale.code')" prop="code">
          <el-input v-model="form.code" :disabled="this.$route.params.code ? true : false" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.content')" prop="message">
          <el-input type="textarea" v-model="form.message" clearable required/>
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

export default {
  name: 'UsersProfile',
  data() {
    return {
      title: this.$t("nut.admin.locales.index.title"),
      form: {
        code: '',
        message: '',
      },
      rules: {
        code: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        message: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }]
      }
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'admin.locales.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          client().request(`mutation form($code: String!, $message: String!){
            updateLocale(code: $code, message: $message) {
              createdAt
            }
          }`, {
            code: this.form.code,
            message: this.form.message
          }).then(() => {
            this.$message({
              type: 'success',
              message: this.$t("flashes.success")
            })
            this.go_back()
          }).catch(failed)
        } else {
          return false;
        }
      });
    },
    init() {
      var code = this.$route.params.code
      if (code) {
        this.form.code = code
        client().request(`query info($code: String!){
        getLocale(code: $code) {
          message
        }
      }`, {
          code
        }).then((rst) => {
          var ifo = rst.getLocale
          this.form.message = ifo.message
        }).catch(failed)
      }
    }
  }
}
</script>
