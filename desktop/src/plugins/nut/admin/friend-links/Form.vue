<template>
<dashboard-layout :title="title" :role="null" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.title')" prop="title">
          <el-input v-model="form.title" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.url')" prop="home">
          <el-input v-model="form.home" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.logo')" prop="logo">
          <el-input v-model="form.logo" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.position')">
          <el-select v-model="form.position">
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

export default {
  name: 'AdminFriendLinksForm',
  data() {
    return {
      title: this.$t("nut.admin.friend-links.index.title"),
      form: {
        id: null,
        title: '',
        home: '',
        logo: '',
        position: 0,
      },
      rules: {
        title: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        home: [{
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
  computed: {
    sort_orders() {
      return lodash.range(-9, 10)
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'admin.friend-links.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $title: String!, $home: String!, $logo: String!, $position: Int!){
            updateFriendLink(id: $id, title: $title, home: $home, logo: $logo, position: $position) {
              createdAt
            }
          }` :
            `mutation form($title: String!, $home: String!, $logo: String!, $position: Int!){
            createFriendLink(title: $title, home: $home, logo: $logo, position: $position) {
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
        showFriendLink(id: $id) {
          id, title, home, logo, position
        }
      }`, {
          id
        }).then((rst) => {
          this.form = rst.showFriendLink
        }).catch(failed)
      }
    }
  }
}
</script>
