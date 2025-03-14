#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
DL_URL="https://github.com/adobe-fonts/source-han-sans/releases/download/2.004R/SourceHanSansSC.zip"

# Download and extract
mkdir -p collect/share/fonts

curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip -j -o ${PROJ}.zip -d collect/share/fonts '*.otf' ||
    { echo "Error: Failed to extract ${PROJ}"; exit 1; }

# Use build_tar function from common.sh
build_tar
