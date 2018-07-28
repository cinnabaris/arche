#!/bin/sh

set -e

rustup update
cargo update
rustup component add rls-preview rust-analysis rust-src rustfmt-preview clippy-preview
cargo install --force diesel_cli
make clean
cargo check
cargo build
make
cargo doc
