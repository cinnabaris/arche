<template>
<el-menu :collapse="$store.state.siderBar" background-color="#545c64" text-color="#fff" active-text-color="#ffd04b">
  <el-submenu :key="it.label" v-for="it in menus" :index="it.label">
    <template slot="title">
        <icon-font :name="it.icon"/>&nbsp;
        <span>{{$t(it.label)}}</span>
      </template>
    <el-menu-item :key="jt.to" v-for="jt in it.children" :index="it.label+'-'+jt.label">
      {{$t(jt.label)}}
    </el-menu-item>
  </el-submenu>
</el-menu>
</template>

<script>
import {
  check,
  ADMIN,
  CARING,
  FORUM,
  POS,
  LIBRARY,
  HOTEL
} from '@/authorized'

export default {
  name: 'Sider',
  computed: {
    menus() {
      var user = this.$store.state.currentUser
      var items = [{
        icon: 'user',
        label: 'nut.users.dashboard.title',
        children: [{
          label: 'nut.users.logs.title',
          to: 'users.logs'
        }, {
          label: 'nut.users.profile.title',
          to: 'users.profile'
        }, {
          label: 'nut.users.change-password.title',
          to: 'users.change-password'
        }]
      }]
      if (check(user, ADMIN)) {
        items.push({
          icon: 'setting',
          label: 'nut.admin.dashboard.title',
          children: []
        })
      }
      var forum = {
        icon: 'sharealt',
        label: 'forum.dashboard.title',
        children: []
      }
      if (check(user, FORUM)) {
        // TODO
      }
      items.push(forum)
      items.push({
        icon: 'book',
        label: 'cbeta.dashboard.title',
        children: []
      })
      if (check(user, LIBRARY)) {
        items.push({
          icon: 'idcard',
          label: 'library.dashboard.title',
          children: []
        })
      }
      if (check(user, HOTEL)) {
        items.push({
          icon: 'fork',
          label: 'hotel.dashboard.title',
          children: []
        })
      }

      items.push({
        icon: 'shoppingcart',
        label: 'shop.dashboard.title',
        children: []
      })

      if (check(user, POS)) {
        items.push({
          icon: 'qrcode',
          label: 'pos.dashboard.title',
          children: []
        })
      }

      items.push({
        icon: 'paperclip',
        label: 'todo.dashboard.title',
        children: []
      })


      if (check(user, CARING)) {
        items.push({
          icon: 'team',
          label: 'caring.dashboard.title',
          children: []
        })
      }
      if (check(user, ADMIN)) {
        items.push({
          icon: 'bank',
          label: 'donate.dashboard.title',
          children: []
        })
        items.push({
          icon: 'USB',
          label: 'ops.vpn.dashboard.title',
          children: []
        })
        items.push({
          icon: 'mail',
          label: 'ops.email.dashboard.title',
          children: []
        })
      }
      return items
    }
  }

}
</script>
