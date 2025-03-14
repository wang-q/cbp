#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-mac.zip"
elif [ "$OS_TYPE" == "windows" ]; then
    DL_URL="https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-win.zip"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}".zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }

unzip ${PROJ}.zip

# Collect binaries
collect_bins ninja

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/ninja" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
