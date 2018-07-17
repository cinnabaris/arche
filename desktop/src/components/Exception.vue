<template>
<el-container>
  <el-main>
    <el-row>
      <el-col :md="{offset:8, span:8}" :sm="{span: 24}">
        <el-alert :title="reason" type="error" show-icon>
          &nbsp;
          <el-button type='text' @click="() => this.$router.push(button.to)">{{button.label}}</el-button>
        </el-alert>
      </el-col>
    </el-row>
  </el-main>
</el-container>
</template>

<script>
export default {
  name: 'Exception',
  props: {
    button: {
      to: Object,
      label: String,
    },
    reason: String,
  }
}
</script>
