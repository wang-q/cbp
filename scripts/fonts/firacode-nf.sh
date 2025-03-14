#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
DL_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0/FiraCode.zip"

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip ${PROJ}.zip -d ${PROJ} ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

mkdir -p collect/share/fonts
mv ${PROJ}/*.ttf collect/share/fonts/

# Use build_tar function from common.sh
build_tar
