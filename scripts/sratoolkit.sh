#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/sra/sdk/3.2.0/sratoolkit.3.2.0-centos_linux64.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/sra/sdk/3.2.0/sratoolkit.3.2.0-mac-arm64.tar.gz"
elif [ "$OS_TYPE" == "windows" ]; then
    DL_URL="https://ftp.ncbi.nlm.nih.gov/sra/sdk/3.2.0/sratoolkit.3.2.0-win64.zip"
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

# Remove symbolic links and rename binaries
find sratoolkit.*/bin -type l -delete
for f in sratoolkit.*/bin/*.[0-9].[0-9].[0-9]; do
    mv "$f" "${f%.[0-9].[0-9].[0-9]}"
done
for f in sratoolkit.*/bin/*-orig; do
    mv "$f" "${f%-orig}"
done

# Collect binaries
mkdir -p collect/
mv sratoolkit.*/bin collect/

# eza -T .

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/fastq-dump" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar
