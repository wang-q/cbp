#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/eBay/tsv-utils/releases/download/v2.1.1/tsv-utils-v2.1.1_linux-x86_64_ldc2.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    DL_URL="https://github.com/eBay/tsv-utils/releases/download/v2.1.1/tsv-utils-v2.1.1_osx-x86_64_ldc2.tar.gz"
fi

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.tar.gz ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvfz ${PROJ}.tar.gz ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Collect binaries
mkdir -p collect/bin
cp tsv-utils*/bin/* collect/bin/
cp tsv-utils*/extras/scripts/* collect/bin/

# Use build_tar function from common.sh
build_tar
