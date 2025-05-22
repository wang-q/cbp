#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/header.sh"

remove_package() {
    local pkg="$1"
    local record_file="${CBP_RECORDS}/${pkg}.files"

    if [ ! -f "${record_file}" ]; then
        echo "==> Package ${pkg} is not installed"
        return 1
    fi

    echo "==> Removing ${pkg}"

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
