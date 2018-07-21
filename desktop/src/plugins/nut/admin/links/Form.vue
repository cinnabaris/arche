<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.name')" prop="label">
          <el-input v-model="form.label" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.url')" prop="href">
          <el-input v-model="form.href" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.loc')">
          <el-select v-model="form.loc">
            <el-option v-for="it in ['header', 'footer', 'sider']" :key="it" :label="it" :value="it" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('attributes.x')">
          <el-select v-model="form.x">
            <el-option v-for="it in sort_orders" :key="it" :label="it" :value="it" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('attributes.y')">
          <el-select v-model="form.y">
            <el-option v-for="it in sort_orders" :key="it" :label="it" :value="it" />
          </el-select>
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
import lodash from 'lodash'
import {
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminLinksForm',
  data() {
    return {
      role: {
        name: ADMIN
      },
      title: this.$t("nut.admin.links.index.title"),
      form: {
        id: null,
        label: '',
        href: '',
        loc: 'header',
        x: 0,
        y: 0,
      },
      rules: {
        label: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        href: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }]
      }
    }
  },
  computed: {
    sort_orders() {
      return lodash.range(-9, 10)
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'admin.links.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $label: String!, $href: String!, $loc: String!, $x: Int!, $y: Int!){
            updateLink(id: $id, label: $label, href: $href, loc: $loc, x: $x, y: $y) {
              createdAt
            }
          }` :
            `mutation form($label: String!, $href: String!, $loc: String!, $x: Int!, $y: Int!){
            createLink(label: $label, href: $href, loc: $loc, x: $x, y: $y) {
              createdAt
            }
          }`,
            this.form).then(() => {
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
      var id = this.$route.params.id
      if (id) {
        client().request(`query info($id: String!){
        showLink(id: $id) {
          id, label, href, loc, x, y
        }
      }`, {
          id
        }).then((rst) => {
          this.form = rst.showLink
        }).catch(failed)
      }
    }
  }
}
</script>
