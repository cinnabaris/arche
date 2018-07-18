import Vue from 'vue'
import Vuex from 'vuex'
import jwtDecode from 'jwt-decode'
import moment from 'moment'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    currentUser: null,
    siderBar: {
      show: true,
      active: null
    },
    siteInfo: {
      title: '',
      subhead: '',
      copyright: ''
    }
  },
  mutations: {
    refresh(state, info) {
      state.siteInfo = info
    },
    selectSiderBar(state, key) {
      state.siderBar.active = key
    },
    toggleSiderBar(state) {
      state.siderBar.show = !state.siderBar.show
    },
    signIn(state, token) {
      try {
        var it = jwtDecode(token);
        if (moment().isBetween(moment.unix(it.nbf), moment.unix(it.exp))) {
          state.currentUser = {
            uid: it.uid,
            groups: it.groups
          }
        }
      } catch (e) {
        // eslint-disable-next-line
        console.error(e)
      }
    },
    signOut(state) {
      state.currentUser = null
    }
  }
})

export default store
