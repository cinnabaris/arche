<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.body')" prop="body">
          <quill-editor v-model="form.body" />
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
  name: 'ForumPostForm',
  data() {
    return {
      title: this.$t("forum.posts.index.title"),
      role: null,
      form: {
        id: null,
        body: '',
        mediaType: 'html',
      },
      rules: {
        // body: [{
        //   required: true,
        //   message: this.$t('validations.required'),
        //   trigger: ['blur', 'change']
        // }]
      }
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'forum.posts.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          var topic = this.$route.params.topic
          client().request(id ?
            `mutation form($id: String!, $body: String!, $mediaType: String!){
            updateForumPost(id: $id, body: $body, mediaType: $mediaType) {
              createdAt
            }
          }` :
            `mutation form($topicId: String!, $body: String!, $mediaType: String!){
            createForumPost(topicId: $topicId, body: $body, mediaType: $mediaType) {
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
          showForumPost(id: $id) {
            id, body, mediaType
          }
        }`, {
          id
        }).then((rst) => {
          this.form = rst.showForumPost
        }).catch(failed)
      } else {
        this.form.topicId = this.$route.params.topic
      }
    }
  }
}
</script>
