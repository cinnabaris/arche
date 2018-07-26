export default {
  "proxy": {
    "/graphql": {
      "target": "http://localhost:8080/"
    }
  },
  plugins: [
    ['umi-plugin-dva', {
      immer: true
    }],
  ]
};