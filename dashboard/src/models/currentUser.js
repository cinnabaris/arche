export default {
  namespace: 'currentUser',
  state: {
    uid: null,
    roles: []
  },
  reducers: {
    signIn(state, token) {
      // TODO
      return {
        uid: 'aaa',
        roles: ['admin']
      }
    },
    signOut(state) {
      return {
        uid: null,
        roles: []
      }
    }
  },
};