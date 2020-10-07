#!/bin/bash

{
    args=("$@")
    LIB_VERSION=${args[0]}

    VERSIONS=$(cargo whatfeatures -l ninres -s)

    VERION_EXISTS=0

    while IFS=$'\n' read -ra VERSION; do
        for V in "${VERSION[@]}"; do
            if [ "$(echo ninres = ${LIB_VERSION})" = "$V" ]; then
                VERION_EXISTS=1
                break
            fi
        done
    done <<< "$VERSIONS"
} >/dev/null 2>&1

echo $VERION_EXISTS
