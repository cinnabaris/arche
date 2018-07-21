<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="100px">
        <el-form-item :label="$t('nut.attributes.seo.google')" prop="google">
          <el-input v-model="form.google" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.seo.baidu')" prop="baidu">
          <el-input v-model="form.baidu" clearable required/>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm('form')">{{$t('buttons.submit')}}</el-button>
        </el-form-item>
      </el-form>

      <el-menu @select="handleSelect">
        <el-menu-item :key="it" v-for="it in items" :index="it">
          <span slot="title">{{it}}</span>
        </el-menu-item>
      </el-menu>
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
import locales from '@/locales'

export default {
  name: 'AdminSiteSeo',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.site.seo.title"),
      form: {
        google: '',
        baidu: ''
      },
      rules: {}
    }
  },
  computed: {
    items() {
      return Object.keys(locales).map((it) => `/rss/${it}`).concat([
        `/google${this.form.google}.html`,
        `/baidu_verify_${this.form.baidu}.html`,
        '/robots.txt',
        '/sitemap.xml'
      ])
    }
  },
  methods: {
    handleSelect(key) {
      var win = window.open(key, '_blank')
      win.focus()
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          client().request(
            `mutation form($google: String!, $baidu: String!){
            updateSiteSeo(google: $google, baidu: $baidu) {
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
        getSiteSeo{
          google, baidu
        }
      }`, {}).then((rst) => {
        this.form = rst.getSiteSeo
      }).catch(() => {})
    }
  }
}
</script>
