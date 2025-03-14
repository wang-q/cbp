#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/marbl/Mash/releases/download/v2.3/mash-Linux64-v2.3.tar"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/marbl/Mash/releases/download/v2.3/mash-OSX64-v2.3.tar"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download and extract
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}.tar" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvf "${PROJ}.tar" ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
collect_bins mash-*/mash

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/mash" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
