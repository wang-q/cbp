#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-macos-arm64"
elif [ "$OS_TYPE" == "windows" ]; then
    DL_URL="https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-windows-amd64.exe"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }

# Collect binaries
collect_bins jq

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/jq" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
