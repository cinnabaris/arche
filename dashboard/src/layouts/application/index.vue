<template>
<v-app>
  <v-content>
    <v-container fluid fill-height>
      <v-layout align-center justify-center>
        <v-flex xs12 sm8 md4>
          <v-card class="elevation-12">
            <v-toolbar dark color="primary">
              <v-toolbar-title>{{ title }}</v-toolbar-title>
              <v-spacer/>
              <v-tooltip bottom>
                <v-btn slot="activator" href="https://github.com/cinnabaris/arche" icon large target="_blank">
                  <v-icon large>mdi-codepen</v-icon>
                </v-btn>
                <span>{{$t('layouts.application.code')}}</span>
              </v-tooltip>
              <v-tooltip right>
                <v-btn slot="activator" icon large href="/" target="_blank">
                  <v-icon large>home</v-icon>
                </v-btn>
                <span>{{$t('layouts.application.home')}}</span>
              </v-tooltip>
            </v-toolbar>
            <v-card-text>
              <v-form>
                <slot/>
              </v-form>
            </v-card-text>
            <v-card-actions>
              <v-spacer/>
              <v-btn v-on:click="onSubmit" color="primary">{{ $t("buttons.submit") }}</v-btn>
            </v-card-actions>
            <v-divider/>
            <div class="text-xs-center">
              <v-chip v-on:click="$router.push({name:it.to})" v-for="it in links" :key="it.to">
                <v-avatar>
                  <v-icon>{{it.icon}}</v-icon>
                </v-avatar>
                {{$t(`${it.to}.title`)}}
              </v-chip>
            </div>
          </v-card>
        </v-flex>
      </v-layout>
    </v-container>
  </v-content>
</v-app>
</template>

<script>
export default {
  name: 'LayoutApplication',
  props: ['title', 'onSubmit'],
  data() {
    return {
      links: [{
        icon: 'mdi-login-variant',
        to: 'users.sign-in'
      }, {
        icon: 'mdi-account-plus-outline',
        to: 'users.sign-up'
      }, {
        icon: 'mdi-find-replace',
        to: 'users.forgot-password'
      }, {
        icon: 'mdi-checkbox-marked-outline',
        to: 'users.confirm'
      }, {
        icon: 'mdi-lock-open-outline',
        to: 'users.unlock'
      }, {
        icon: 'mdi-comment-question-outline',
        to: 'leave-words.new'
      }]
    }
  }
}
</script>
