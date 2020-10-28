#!/bin/bash

if [[ $CRATE_VERSION_EXISTS == 1 ]]; then
    exit 0
fi

args=("$@")
CRATE_PATH=${args[0]}

# Crates.io publish
cd $CRATE_PATH && cargo publish --token \"$CARGO_CREDENTIALS\"
