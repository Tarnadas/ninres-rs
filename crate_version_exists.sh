#!/bin/bash

{
    args=("$@")
    LIB_VERSION=${args[0]}
    CRATE_PATH=${args[1]}

    VERSIONS=$(cd $CRATE_PATH && cargo whatfeatures -l $CRATE_PATH)

    VERION_EXISTS=0

    while IFS=$'\n' read -ra VERSION; do
        for V in "${VERSION[@]}"; do
            V=$(cut -d '#' -f 1 <<< "$V")
            V=$(sed 's/[",($CRATE_PATH = )]//g' <<< $V)
            if [ "$LIB_VERSION" = "$V" ]; then
                VERION_EXISTS=1
                break
            fi
        done
    done <<< "$VERSIONS"
} >/dev/null 2>&1

echo $VERION_EXISTS
