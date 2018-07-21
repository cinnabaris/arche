import Vue from 'vue'
import Vuex from 'vuex'
import jwtDecode from 'jwt-decode'
import moment from 'moment'

import {removeToken, setToken} from './utils'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    currentUser: {},
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
    updatePolicies(state, policies) {
      state.currentUser.policies = policies
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
          state.currentUser.uid = it.uid
        }
        setToken(token)
        return
      } catch (e) {
        // eslint-disable-next-line
        console.error(e)
      }
      removeToken()
    },
    signOut(state) {
      state.currentUser = {}
      removeToken()
    }
  }
})

export default store
