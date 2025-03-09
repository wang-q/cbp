#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/blast/executables/blast+/2.16.0/ncbi-blast-2.16.0+-x64-linux.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/blast/executables/blast+/2.16.0/ncbi-blast-2.16.0+-aarch64-macosx.tar.gz"
elif [ "$OS_TYPE" == "windows" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/blast/executables/blast+/2.16.0/ncbi-blast-2.16.0+-x64-win64.tar.gz"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}.tar.gz" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvfz "${PROJ}.tar.gz" ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
collect_bins ncbi-blast-*/bin/*

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/blastn" -version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
