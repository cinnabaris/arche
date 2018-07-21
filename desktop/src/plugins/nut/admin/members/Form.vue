<template>
<dashboard-layout :title="title" :role="role" :init="init">
  <col-form>
    <el-card :header="title">
      <el-form :rules="rules" ref="form" :model="form" label-width="80px">
        <el-form-item :label="$t('nut.attributes.member.nick-name')" prop="nickName">
          <el-input :disabled="$route.params.id ? true : false" v-model="form.nickName" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.real-name')" prop="realName">
          <el-input v-model="form.realName" clearable required/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.phone')" prop="phone">
          <el-input v-model="form.phone" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.email')" prop="email">
          <el-input v-model="form.email" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.address')" prop="address">
          <el-input type="textarea" v-model="form.address" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.line')" prop="line">
          <el-input v-model="form.line" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.wechat')" prop="wechat">
          <el-input v-model="form.wechat" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.skype')" prop="skype">
          <el-input v-model="form.skype" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.weibo')" prop="weibo">
          <el-input v-model="form.weibo" clearable/>
        </el-form-item>
        <el-form-item :label="$t('nut.attributes.member.facebook')" prop="facebook">
          <el-input v-model="form.facebook" clearable/>
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
  name: 'AdminMembersForm',
  data() {
    return {
      role: {
        name: ADMIN,
        type: null,
        id: null
      },
      title: this.$t("nut.admin.members.index.title"),
      form: {
        id: null,
        nickName: '',
        realName: '',
        phone: '',
        email: '',
        address: '',
        line: '',
        wechat: '',
        skype: '',
        weibo: '',
        facebook: ''
      },
      rules: {
        nickName: [{
          required: true,
          message: this.$t('validations.required'),
          trigger: ['blur', 'change']
        }],
        realName: [{
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
        name: 'admin.members.index'
      })
    },
    submitForm(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          var id = this.$route.params.id
          client().request(id ?
            `mutation form($id: String!, $realName: String!, $phone: String, $email: String, $address: String, $line: String, $wechat: String, $skype: String, $weibo: String, $facebook: String){
            updateMember(id: $id, realName: $realName, phone: $phone, email: $email, address: $address, line: $line, wechat: $wechat, skype: $skype, weibo: $weibo, facebook: $facebook) {
              createdAt
            }
          }` :
            `mutation form($nickName: String!, $realName: String!, $phone: String, $email: String, $address: String, $line: String, $wechat: String, $skype: String, $weibo: String, $facebook: String){
            createMember(nickName: $nickName, realName: $realName, phone: $phone, email: $email, address: $address, line: $line, wechat: $wechat, skype: $skype, weibo: $weibo, facebook: $facebook) {
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
        showMember(id: $id) {
          id, nickName, realName, phone, email, address, line, wechat, skype, weibo, facebook
        }
      }`, {
          id
        }).then((rst) => {
          this.form = rst.showMember
        }).catch(failed)
      }
    }
  }
}
</script>
