#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/iqtree/iqtree2/releases/download/v2.4.0/iqtree-2.4.0-Linux-intel.tar.gz"
    FILE_EXT="tar.gz"
    EXTRACT_CMD="tar xvfz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/iqtree/iqtree2/releases/download/v2.4.0/iqtree-2.4.0-macOS.zip"
    FILE_EXT="zip"
    EXTRACT_CMD="unzip"
fi

# Download and extract
mkdir -p "${TEMP_DIR}"
cd "${TEMP_DIR}"

curl -L ${DL_URL} -o iqtree.${FILE_EXT} ||
    { echo "Error: Failed to download iqtree2"; exit 1; }
${EXTRACT_CMD} iqtree.${FILE_EXT} ||
    { echo "Error: Failed to extract iqtree2"; exit 1; }

# Collect binaries
mkdir -p collect/bin
cp iqtree-*/bin/* collect/bin/

# Use build_tar function from common.sh
build_tar
