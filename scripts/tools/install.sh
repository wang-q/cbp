#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

download_package() {
    local pkg_name="$1"
    local temp_file="${CBP_CACHE}/${pkg_name}.${OS_TYPE}.tar.gz.incomplete"
    
    ensure_dirs
    
    if ! gh release download Binaries -p "${pkg_name}.${OS_TYPE}.tar.gz" -O "${temp_file}"; then
        echo "    Failed to download ${pkg_name} for ${OS_TYPE}"
        return 1
    fi
    
    mv "${temp_file}" "${CBP_CACHE}/${pkg_name}.${OS_TYPE}.tar.gz"
    return 0
}

install_package() {
    local pkg_name="$1"
    local pkg_file="${CBP_CACHE}/${pkg_name}.${OS_TYPE}.tar.gz"

    echo "==> Installing ${pkg_name}"
    ensure_dirs

    # List files in package
    tar tzf "${pkg_file}" > "${CBP_BINARIES}/${pkg_name}.files" || {
        echo "    Failed to list files in ${pkg_name}"
        return 1
    }

    # Extract files
    tar xzf "${pkg_file}" --directory="${CBP_HOME}" || {
        echo "    Failed to extract ${pkg_name}"
        rm -f "${CBP_BINARIES}/${pkg_name}.files"
        return 1
    }

    echo "    Done"
    return 0
}

# Process packages
for pkg in "$@"; do
    if download_package "${pkg}"; then
        install_package "${pkg}" "${CBP_CACHE}/${pkg}.${OS_TYPE}.tar.gz" || exit 1
    else
        exit 1
    fi
done
