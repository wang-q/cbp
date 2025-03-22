#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/ericchiang/pup/releases/download/v0.4.0/pup_v0.4.0_linux_amd64.zip"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/ericchiang/pup/releases/download/v0.4.0/pup_v0.4.0_darwin_amd64.zip"
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip ${PROJ}.zip ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Use build_tar function from common.sh
collect_bins pup
build_tar
