#!/bin/sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
bash -s -- -y --no-modify-path

# set env to path
. "$HOME/.cargo/env"

# add targets
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none
