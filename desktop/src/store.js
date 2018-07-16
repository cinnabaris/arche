import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    currentUser: null
  },
  mutations: {
    signIn(state, token) {
      // TODO
      state.currentUser = {
        uid: ''
      }
    },
    signOut(state) {
      state.currentUser = null
    }
  }
})

export default store
