#!/bin/bash
#
# Usage: ./build_in_github
#
# The latest version of this script is available at
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release

set -euo pipefail

photo-studio-musl() {
    docker run --rm -i -v "$(pwd)":/home/rust/src -v cargo-registry:/home/rust/.cargo/registry ekidd/photo-studio-musl "$@"
}

echo "Building static binaries using ekidd/photo-studio-musl"
photo-studio-musl sudo chown -R rust:rust /home/rust
photo-studio-musl sudo chown -R rust:rust /home/rust/.cargo/registry
photo-studio-musl cargo build --release

