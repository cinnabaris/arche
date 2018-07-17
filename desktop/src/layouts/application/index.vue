<template>
<el-container>
  <el-header/>
  <el-main>
    <el-row>
      <el-col :md="{offset:8, span:8}" :sm="{span: 24}">
        <document-title :title="title" />
        <slot/>
      </el-col>
    </el-row>
  </el-main>
  <layout-footer/>
</el-container>
</template>

<script>
import Footer from '../Footer'
export default {
  name: 'ApplicationLayout',
  props: {
    title: String
  },
  components: {
    'layout-footer': Footer
  }
}
</script>
