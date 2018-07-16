module.exports = {
  baseUrl: '/my/',
  devServer: {
    port: 3000,
    proxy: {
      '/graphql': {
        target: 'http://localhost:8080'
      }
    }
  }
}
