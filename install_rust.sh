#!/bin/sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
bash -s -- -y --no-modify-path

. "$HOME/.cargo/env"
