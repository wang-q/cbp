#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
DL_URL="https://github.com/mozilla/Fira/archive/4.202.tar.gz"

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.tar.gz ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
tar xvfz ${PROJ}.tar.gz ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

mkdir -p collect/share/fonts
mv Fira-4.202/ttf/* collect/share/fonts/

# Use build_tar function from common.sh
build_tar
