#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/BenLangmead/bowtie2/releases/download/v2.5.4/bowtie2-2.5.4-linux-x86_64.zip"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/BenLangmead/bowtie2/releases/download/v2.5.4/bowtie2-2.5.4-macos-arm64.zip"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}.zip" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip "${PROJ}.zip" ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
rm bowtie2-*/*-debug
collect_bins bowtie2-*/bowtie2*

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/bowtie2" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
