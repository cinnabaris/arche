<template>
<el-container>
  <el-header/>
  <el-main>
    <el-row>
      <el-col :md="{offset:8, span:8}" :sm="{span: 24}">
        <slot/>
      </el-col>
    </el-row>
  </el-main>
  <el-footer/>
</el-container>
</template>

<script>
export default {
  name: 'ApplicationLayout',
  props: {}
}
</script>
