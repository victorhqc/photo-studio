FROM ekidd/rust-musl-builder

RUN sudo apt-get update && sudo apt-get install libsqlite3-dev
