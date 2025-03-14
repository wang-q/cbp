#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/rcedgar/muscle/releases/download/v5.3/muscle-linux-x86.v5.3"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }

# Collect binaries
collect_bins "${PROJ}"

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/muscle" -version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
