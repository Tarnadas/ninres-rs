#!/bin/bash

{
    args=("$@")
    LIB_VERSION=${args[0]}

    VERSIONS=$(cd ninres && cargo whatfeatures -l ninres)

    VERION_EXISTS=0

    while IFS=$'\n' read -ra VERSION; do
        for V in "${VERSION[@]}"; do
            V=$(cut -d '#' -f 1 <<< "$V")
            V=$(sed 's/[",(ninres = )]//g' <<< $V)
            if [ "$LIB_VERSION" = "$V" ]; then
                VERION_EXISTS=1
                break
            fi
        done
    done <<< "$VERSIONS"
} >/dev/null 2>&1

echo $VERION_EXISTS
