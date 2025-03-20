#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_macos.zip"
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
collect_bins raxml-ng

# Pack binaries
build_tar
