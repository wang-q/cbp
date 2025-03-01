#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

remove_package() {
    local pkg_name="$1"
    local record_file="${CBP_BINARIES}/${pkg_name}.files"

    if [ ! -f "${record_file}" ]; then
        echo "==> Package ${pkg_name} is not installed"
        return 1
    fi

    echo "==> Removing ${pkg_name}"

    # Remove files in package
    while read -r file; do
        if [ -f "${CBP_HOME}/${file}" ]; then
            rm -f "${CBP_HOME}/${file}"
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
