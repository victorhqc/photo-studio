#!/bin/bash
#
# Usage: ./build <PROJECT> ${VERSION}
# Example: ./build photo-api 0.1.0
#
# The latest version of this script is available at
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release

set -euo pipefail

photo-studio-musl() {
    docker run --rm -it -v "$(pwd)":/home/rust/src -v cargo-registry:/home/rust/.cargo/registry photo-studio-musl "$@"
}

echo "Building static binaries using photo-studio-musl"
photo-studio-musl sudo chown -R rust:rust /home/rust/.cargo/registry
photo-studio-musl cargo build --release
zip -j "$1"-"$2".zip target/x86_64-unknown-linux-musl/release/"$1"
