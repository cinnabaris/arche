<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.title')" prop="title">
          <el-input v-model="form.title" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.subhead')" prop="subhead">
          <el-input v-model="form.subhead" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.keywords')" prop="keywords">
          <el-input v-model="form.keywords" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.description')" prop="description">
          <el-input type="textarea" v-model="form.description" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.copyright')" prop="copyright">
          <el-input v-model="form.copyright" clearable required/>
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
  name: 'AdminSiteInfo',
  data() {
    return {
      title: this.$t("nut.admin.site.info.title"),
      form: {
        title: '',
        subhead: '',
        keywords: '',
        description: '',
        copyright: ''
      },
      rules: {
        title: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        subhead: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        keywords: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        description: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        copyright: [{
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
          client().request(
            `mutation form($title: String!, $subhead: String!, $keywords: String!, $description: String!, $copyright: String!){
            updateSiteInfo(title: $title, subhead: $subhead, keywords: $keywords, description: $description, copyright: $copyright) {
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
        getSiteInfo{
          title, subhead, keywords, description, copyright
        }
      }`, {}).then((rst) => {
        this.form = rst.getSiteInfo
      }).catch(failed)
    }
  }
}
</script>
