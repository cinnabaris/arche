#!/bin/sh

rustup update
cargo install --force racer
cargo install --force rustfmt-nightly
cargo install diesel_cli --force --no-default-features --features postgres,mysql
cargo update
cargo build
cargo build --release

# npx create-react-app dashboard
# npm install --save react react-dom antd emoji-mart js-cookie jwt-decode moment moment-timezone qrcode.react react-color react-copy-to-clipboard react-google-maps react-intl react-loadable react-markdown react-moment react-quill react-redux react-router react-router-dom react-syntax-highlighter redux react-router-redux@next react-document-title react-helmet graphql-request
