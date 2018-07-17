<template>
<el-footer style="text-align: center;">
  &copy;copyright
  <el-button :key="it" v-for="(it) in languages" @click="switchLocale(it)" type="text">
    {{$t(`languages.${it}`)}}
  </el-button>
</el-footer>
</template>

<script>
import locales from '@/locales'
import {
  setLocale
} from '@/utils'

export default {
  name: 'Footer',
  data() {
    return {
      languages: Object.keys(locales)
    }
  },
  methods: {
    switchLocale(l) {
      setLocale(l)
      this.$i18n.locale = l
    }
  }
}
</script>
