#!/bin/bash
#
# Usage: ./build <PROJECT> ${VERSION}
# Example: ./build photo-api 0.1.0
#
# The latest version of this script is available at
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release

set -euo pipefail

rust-musl-builder() {
    docker run --rm -it -v "$(pwd)":/home/rust/src -v cargo-registry:/home/rust/.cargo/registry ekidd/rust-musl-builder "$@"
}

echo "Building static binaries using ekidd/rust-musl-builder"
rust-musl-builder sudo chown -R rust:rust /home/rust/.cargo/registry
rust-musl-builder cargo build --release
zip -j "$1"-"$2".zip target/x86_64-unknown-linux-musl/release/"$1"
