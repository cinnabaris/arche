#/bin/sh

# https://www.musl-libc.org/how.html
# https://download.libsodium.org/doc/
# http://www.libressl.org/
# https://www.openssl.org/source/

target="x86_64-unknown-linux-musl"

# rustup target install $target

export SODIUM_LIB_DIR=/usr/local/lib/
export SODIUM_STATIC=true
export OPENSSL_DIR=/usr/local/
export OPENSSL_STATIC=true

cargo build --release --target=$target
strip -s target/$target/release/arche
