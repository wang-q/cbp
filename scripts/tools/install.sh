#!/bin/bash

# Detect platform
if [[ "$(uname)" == "Darwin" ]]; then
    OS_TYPE="macos"
else
    OS_TYPE="linux"
fi

download_package() {
    local pkg_name="$1"
    local cache_dir="$HOME/.cbp/cache"
    local temp_file="${cache_dir}/${pkg_name}.${OS_TYPE}.tar.gz.incomplete"
    
    mkdir -p "${cache_dir}"
    
    if ! gh release download Binaries -p "${pkg_name}.${OS_TYPE}.tar.gz" -O "${temp_file}"; then
        echo "    Failed to download ${pkg_name} for ${OS_TYPE}"
        return 1
    fi
    
    mv "${temp_file}" "${cache_dir}/${pkg_name}.${OS_TYPE}.tar.gz"
    return 0
}

install_package() {
    local pkg_name="$1"
    local install_dir="$HOME/.cbp"
    local record_dir="$HOME/.cbp/binaries"
    local cache_dir="$HOME/.cbp/cache"
    local pkg_file="${cache_dir}/${pkg_name}.${OS_TYPE}.tar.gz"

    echo "==> Installing ${pkg_name}"
    mkdir -p "${record_dir}"

    # List files in package
    tar tzf "${pkg_file}" > "${record_dir}/${pkg_name}.files" || {
        echo "    Failed to list files in ${pkg_name}"
        return 1
    }

    # Extract files
    tar xzf "${pkg_file}" --directory="${install_dir}" || {
        echo "    Failed to extract ${pkg_name}"
        rm -f "${record_dir}/${pkg_name}.files"
        return 1
    }

    echo "    Done"
    return 0
}

# Process packages
for pkg in "$@"; do
    download_package "${pkg}" &&
        install_package "${pkg}" || exit 1
done
