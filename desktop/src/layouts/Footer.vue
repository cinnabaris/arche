<template>
<el-footer style="text-align: center;">
  &copy;{{$store.state.siteInfo.copyright}}
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
import {
  client,
  failed
} from '@/request'

export default {
  name: 'Footer',
  data() {
    return {
      languages: Object.keys(locales)
    }
  },
  created() {
    if (!this.$store.state.siteInfo.title) {
      client().request(`query info{
        getSiteInfo {
          title,
          subhead,
          copyright
        }
      }`, {}).then((rst) => {
        this.$store.commit('refresh', rst.getSiteInfo)
      }).catch(failed)
    }
  },
  methods: {
    switchLocale(l) {
      setLocale(l)
      // this.$i18n.locale = l
      location.reload()
    }
  }
}
</script>
