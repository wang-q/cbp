#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/voutcn/megahit/releases/download/v1.2.9/MEGAHIT-1.2.9-Linux-x86_64-static.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL=""
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.tar.gz ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvfz ${PROJ}.tar.gz ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries and scripts
mkdir -p collect/bin
cp MEGAHIT-*/bin/* collect/bin/

# Use build_tar function from common.sh
build_tar
