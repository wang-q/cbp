#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/marbl/Mash/releases/download/v2.3/mash-Linux64-v2.3.tar"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/marbl/Mash/releases/download/v2.3/mash-OSX64-v2.3.tar"
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.tar ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvf ${PROJ}.tar ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Use build_tar function from common.sh
collect_bins  mash-*/mash
build_tar
