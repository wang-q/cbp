#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/freebayes/freebayes/releases/download/v1.3.6/freebayes-1.3.6-linux-amd64-static.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL=""
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.gz ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
gzip -d ${PROJ}.gz ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
mkdir -p collect/bin
mv ${PROJ}* collect/bin/${PROJ}

# Use build_tar function from common.sh
build_tar
