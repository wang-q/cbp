#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Process packages
for pkg in "$@"; do
    if download_package "${pkg}"; then
        install_package "${pkg}" "${CBP_CACHE}/${pkg}.${OS_TYPE}.tar.gz" || exit 1
    else
        exit 1
    fi
done
