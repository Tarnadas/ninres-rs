#!/bin/bash

if [[ $CRATE_VERSION_EXISTS == 1 ]]; then
    exit 0
fi

# Crates.io login
echo "[registry]" >> ~/.cargo/credentials
echo "token = \"$CARGO_CREDENTIALS\"" >> ~/.cargo/credentials

# Crates.io publish
cargo publish
