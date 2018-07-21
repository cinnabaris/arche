<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('attributes.title')" prop="title">
          <el-input v-model="form.title" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('forum.attributes.topic.tags')">
          <el-checkbox-group v-model="form.tags">
            <el-checkbox :key="it.id" v-for="it in tags" :label="it.id" name="tags">{{it.name}}</el-checkbox>
          </el-checkbox-group>
        </el-form-item>
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

export default {
  name: 'ForumTopicForm',
  data() {
    return {
      title: this.$t("forum.topics.index.title"),
      role: null,
      form: {
        id: null,
        title: '',
        body: '',
        mediaType: 'html',
        tags: []
      },
      tags: [],
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
      }
    }
  },
  methods: {
    go_back() {
      this.$router.push({
        name: 'forum.topics.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $title: String!, $body: String!, $mediaType: String!, $tags: [String!]!){
            updateForumTopic(id: $id, title: $title, body: $body, mediaType: $mediaType, tags: $tags) {
              createdAt
            }
          }` :
            `mutation form($title: String!, $body: String!, $mediaType: String!, $tags: [String!]!){
            createForumTopic(title: $title, body: $body, mediaType: $mediaType, tags: $tags) {
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
        showForumTopic(id: $id) {
          title, body, mediaType, tags{id, name}
        }
        listForumTag{
          id, name
        }
      }`, {
          id
        }).then((rst) => {
          var topic = rst.showForumTopic
          this.form = {
            id: id,
            title: topic.title,
            mediaType: topic.mediaType,
            body: topic.body,
            tags: topic.tags.map((it) => it.id),
          }
          this.tags = rst.listForumTag
        }).catch(failed)
      } else {
        client().request(`query list{
        listForumTag{
          id, name
        }
      }`, {}).then((rst) => {
          this.tags = rst.listForumTag
        }).catch(failed)
      }
    }
  }
}
</script>
