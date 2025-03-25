#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/header.sh"

list_unbuilt() {
    echo "==> Packages in scripts/ but not built for ${OS_TYPE}:"
    comm -23 \
        <(cd "${BASH_DIR}/.." && find . -maxdepth 1 -name "*.sh" ! -name "common.sh" ! -name "rust.sh" ! -name "init.sh" -print |
            sed 's/^\.\///' |
            sed 's/\.sh$//' |
            sort) \
        <(gh release download Binaries --pattern "cbp-packages.json" --output - |
            jq -r '.[] | select(.name | endswith(".'"${OS_TYPE}"'.tar.gz")) | .name' |
            sed "s/\.${OS_TYPE}\.tar\.gz$//" |
            sort) |
        perl -n -e "${PERL_FMT}"
    echo
}

list_pkg() {
    echo "==> Packages in pakages/ but not built for ${OS_TYPE}:"
    comm -23 \
        <(cd "${BASH_DIR}/../../packages" && find . -maxdepth 1 -name "*.json" -print |
            sed 's/^\.\///' |
            sed 's/\.json$//' |
            sort) \
        <(gh release download Binaries --pattern "cbp-packages.json" --output - |
            jq -r '.[] | select(.name | endswith(".'"${OS_TYPE}"'.tar.gz")) | .name' |
            sed "s/\.${OS_TYPE}\.tar\.gz$//" |
            sort) |
        perl -n -e "${PERL_FMT}"
    echo
}

list_pkg_rev() {
    echo "==> Packages not in pakages/"
    comm -23 \
        <(gh release download Binaries --pattern "cbp-packages.json" --output - |
            jq -r '.[] | select(.name | endswith(".tar.gz")) | .name' |
            sed "s/\.linux\.tar\.gz$//" |
            sed "s/\.macos\.tar\.gz$//" |
            sed "s/\.windows\.tar\.gz$//" |
            sed "s/\.font\.tar\.gz$//" |
            sort |
            uniq) \
        <(cd "${BASH_DIR}/../../packages" && find . -maxdepth 1 -name "*.json" -print |
            sed 's/^\.\///' |
            sed 's/\.json$//' |
            sort) |
        perl -n -e "${PERL_FMT}"
    echo
}

list_unbuilt

list_pkg

list_pkg_rev
