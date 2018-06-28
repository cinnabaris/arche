<template>
<application-layout>
  <v-card class="elevation-12">
    <v-toolbar dark color="primary">
      <v-toolbar-title>{{ $t("users.sign-in.title") }}</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-tooltip bottom>
        <v-btn slot="activator" :href="source" icon large target="_blank">
          <v-icon large>code</v-icon>
        </v-btn>
        <span>Source</span>
      </v-tooltip>
      <v-tooltip right>
        <v-btn slot="activator" icon large href="https://codepen.io/johnjleider/pen/wyYVVj" target="_blank">
          <v-icon large>mdi-codepen</v-icon>
        </v-btn>
        <span>Codepen</span>
      </v-tooltip>
    </v-toolbar>
    <v-card-text>
      <v-form>
        <v-text-field prepend-icon="person" name="login" label="Login" type="text" />
        <v-text-field id="password" prepend-icon="lock" name="password" label="Password" type="password" />
      </v-form>
    </v-card-text>
    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn color="primary">Login</v-btn>
    </v-card-actions>
  </v-card>
</application-layout>
</template>


<script>
export default {
  name: 'UsersSignIn',
  data: () => ({
    drawer: null
  }),
  props: {
    source: String
  }
}
</script>
