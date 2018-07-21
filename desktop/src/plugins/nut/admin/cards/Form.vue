<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.title')" prop="title">
          <el-input v-model="form.title" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.body')" prop="body">
          <quill-editor v-model="form.body" />
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.card.action')" prop="action">
          <el-input v-model="form.action" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.url')" prop="href">
          <el-input v-model="form.href" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.logo')" prop="logo">
          <el-input v-model="form.logo" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('attributes.loc')">
          <el-select v-model="form.loc">
            <el-option v-for="it in ['top', 'middle', 'bottom']" :key="it" :label="it" :value="it" />
          </el-select>
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
import {
  ADMIN
} from '@/authorized'

export default {
  name: 'AdminCardsForm',
  data() {
    return {
      title: this.$t("nut.admin.cards.index.title"),
      role: {
        name: ADMIN
      },
      form: {
        id: null,
        title: '',
        body: '',
        mediaType: 'html',
        action: '',
        href: '',
        logo: '',
        loc: 'top',
        position: 0,
      },
      rules: {
        title: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        body: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        action: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        href: [{
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
        name: 'admin.cards.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $title: String!, $body: String!, $mediaType: String!, $action: String!, $href: String!, $logo: String!, $loc: String!, $position: Int!){
            updateCard(id: $id, title: $title, body: $body, mediaType: $mediaType, action: $action, href: $href, logo: $logo, loc: $loc, position: $position) {
              createdAt
            }
          }` :
            `mutation form($title: String!, $body: String!, $mediaType: String!, $action: String!, $href: String!, $logo: String!, $loc: String!, $position: Int!){
            createCard(title: $title, body: $body, mediaType: $mediaType, action: $action, href: $href, logo: $logo, loc: $loc, position: $position) {
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
        showCard(id: $id) {
          id, title, body, mediaType, action, href, logo, loc, position
        }
      }`, {
          id
        }).then((rst) => {
          this.form = rst.showCard
        }).catch(failed)
      }
    }
  }
}
</script>
