#!/bin/sh

go get -u github.com/kardianos/govendor

govendor fetch github.com/go-pg/pg
govendor fetch github.com/go-pg/migrations
govendor fetch github.com/gomodule/redigo/redis
govendor fetch github.com/urfave/negroni
govendor fetch github.com/gorilla/mux
govendor fetch github.com/rs/cors
govendor fetch github.com/unrolled/render
govendor fetch github.com/gbrlsnchs/jwt
govendor fetch github.com/nsqio/go-nsq
govendor fetch github.com/graphql-go/graphql
govendor fetch github.com/BurntSushi/toml
govendor fetch github.com/go-ini/ini
govendor fetch golang.org/x/crypto/nacl/secretbox
govendor fetch golang.org/x/crypto/nacl/auth
govendor fetch golang.org/x/crypto/bcrypt
govendor fetch golang.org/x/oauth2
govendor fetch golang.org/x/oauth2/jwt
govendor fetch golang.org/x/oauth2/google
govendor fetch golang.org/x/text/language
govendor fetch github.com/urfave/cli
govendor fetch github.com/facebookgo/inject
govendor fetch github.com/jordan-wright/email
govendor fetch github.com/sirupsen/logrus
govendor fetch github.com/sirupsen/logrus/hooks/syslog
govendor fetch github.com/spf13/viper
govendor fetch github.com/google/uuid
govendor fetch github.com/streadway/amqp
govendor fetch github.com/gorilla/feeds
govendor fetch github.com/ikeikeikeike/go-sitemap-generator/stm
govendor fetch github.com/aws/aws-sdk-go/aws
govendor fetch github.com/aws/aws-sdk-go/aws/awsutil
govendor fetch github.com/aws/aws-sdk-go/aws/credentials
govendor fetch github.com/aws/aws-sdk-go/aws/session
govendor fetch github.com/aws/aws-sdk-go/service/s3


# npm install --save antd ant-design-pro emoji-mart js-cookie jwt-decode moment moment-timezone qrcode.react react-color react-copy-to-clipboard react-google-maps react-intl react-loadable react-markdown react-moment react-quill react-redux react-router react-router-dom react-syntax-highlighter redux react-router-redux@next history react-document-title react-helmet graphql-request
