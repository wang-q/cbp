#!/bin/bash

remove_package() {
    local pkg_name="$1"
    local install_dir="$HOME/.cbp"
    local record_file="$HOME/.cbp/binaries/${pkg_name}.files"

    if [ ! -f "${record_file}" ]; then
        echo "==> Package ${pkg_name} is not installed"
        return 1
    fi

    echo "==> Removing ${pkg_name}"

    # Remove files in package
    while read -r file; do
        if [ -f "${install_dir}/${file}" ]; then
            rm -f "${install_dir}/${file}"
        fi
    done < "${record_file}"

    # Remove record file
    rm -f "${record_file}"
    echo "    Done"
}

# Process packages
for pkg in "$@"; do
    remove_package "${pkg}" || exit 1
done
