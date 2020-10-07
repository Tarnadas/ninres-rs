#!/bin/bash

{
    args=("$@")
    LIB_VERSION=${args[0]}

    VERSIONS=$(cargo whatfeatures -l ninres)
    echo VERSIONS=$VERSIONS

    VERION_EXISTS=0

    while IFS=$'\n' read -ra VERSION; do
    echo VERSION=${VERSION[@]}
        for V in "${VERSION[@]}"; do
            echo V=$V
            echo check=$(echo ninres = ${LIB_VERSION}) eq $V
            if [ "$(echo ninres = ${LIB_VERSION})" = "$V" ]; then
                VERION_EXISTS=1
                break
            fi
        done
    done <<< "$VERSIONS"
} #>/dev/null 2>&1

echo $VERION_EXISTS
