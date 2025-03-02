#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

list_unbuilt() {
    echo "==> Packages in scripts/ but not built for ${OS_TYPE}:"
    comm -23 \
        <(cd "${BASH_DIR}/.." && find . -maxdepth 1 -name "*.sh" ! -name "common.sh" ! -name "rust.sh" ! -name "init.sh" -print | sed 's/^\.\///' | sed 's/\.sh$//' | sort) \
        <(gh release download Binaries --pattern "cbp-packages.json" --output - |
            jq -r '.[] | select(.name | endswith(".'"${OS_TYPE}"'.tar.gz")) | .name' |
            sed "s/\.${OS_TYPE}\.tar\.gz$//" |
            sort) |
        perl -n -e "${PERL_FMT}"
    echo
}

list_unbuilt
