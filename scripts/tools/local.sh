#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/header.sh"

# Process packages
for pkg in "$@"; do
    pkg_file="binaries/${pkg}.${OS_TYPE}.tar.gz"
    if [ ! -f "${pkg_file}" ]; then
        echo "==> Package ${pkg}.${OS_TYPE}.tar.gz not found in binaries/"
        exit 1
    fi
    install_package "${pkg}" "${pkg_file}" || exit 1
done
