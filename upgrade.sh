#!/bin/sh

set -e

rustup update
rustup component add rls-preview rust-analysis rust-src rustfmt-preview
# cargo install --force clippy
cargo install --force diesel_cli
cargo clean
cargo update
cargo check
cargo doc
cargo build
cargo build --release

# npm install --save antd ant-design-pro emoji-mart js-cookie jwt-decode moment moment-timezone qrcode.react react-color react-copy-to-clipboard react-google-maps react-intl react-loadable react-markdown react-moment react-quill react-redux react-router react-router-dom react-syntax-highlighter redux react-router-redux@next history react-document-title react-helmet graphql-request
