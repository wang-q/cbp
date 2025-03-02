#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_macos.zip"
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip ${PROJ}.zip ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
mkdir -p collect/bin
cp raxml-ng collect/bin/

# Use build_tar function from common.sh
build_tar
