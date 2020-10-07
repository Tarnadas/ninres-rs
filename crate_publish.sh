#!/bin/bash

if [[ $CRATE_VERSION_EXISTS == 1 ]]; then
    exit 0
fi

# Crates.io publish
cargo publish --token \"$CARGO_CREDENTIALS\"
