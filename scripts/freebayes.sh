#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/freebayes/freebayes/releases/download/v1.3.6/freebayes-1.3.6-linux-amd64-static.gz"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download and extract
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}.gz" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
gzip -d "${PROJ}.gz" ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
collect_bins ${PROJ}*

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/freebayes" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
