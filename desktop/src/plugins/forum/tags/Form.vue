<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.name')" prop="name">
          <el-input v-model="form.name" clearable required/>
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
import {
  FORUM,
  MANAGER
} from '@/authorized'

export default {
  name: 'TagsForm',
  data() {
    return {
      title: this.$t("forum.tags.index.title"),
      role: {
        name: MANAGER,
        type: FORUM,
        id: null
      },
      form: {
        id: null,
        name: ''
      },
      rules: {
        name: [{
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
        name: 'forum.tags.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $name: String!){
            updateForumTag(id: $id, name: $name) {
              createdAt
            }
          }` :
            `mutation form($name: String!){
            createForumTag(name: $name) {
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
        showForumTag(id: $id) {
          id, name, updatedAt
        }
      }`, {
          id
        }).then((rst) => {
          this.form = rst.showForumTag
        }).catch(failed)
      }
    }
  }
}
</script>
