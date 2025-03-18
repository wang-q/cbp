#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/gpertea/stringtie/releases/download/v3.0.0/stringtie-3.0.0.Linux_x86_64.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/gpertea/stringtie/releases/download/v3.0.0/stringtie-3.0.0.OSX_x86_64.tar.gz"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
if [[ "${DL_URL}" == *.zip ]]; then
    curl -L "${DL_URL}" -o "${PROJ}.zip" ||
        { echo "Error: Failed to download ${PROJ}"; exit 1; }
    unzip "${PROJ}.zip" ||
        { echo "Error: Failed to extract ${PROJ}"; exit 1; }
else
    curl -L "${DL_URL}" -o "${PROJ}.tar.gz" ||
        { echo "Error: Failed to download ${PROJ}"; exit 1; }
    tar xvfz "${PROJ}.tar.gz" ||
        { echo "Error: Failed to extract ${PROJ}"; exit 1; }
fi

# Collect binaries
collect_bins stringtie-*/stringtie

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/stringtie" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
