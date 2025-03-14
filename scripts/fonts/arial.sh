#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Check if cabextract is available
if ! type cabextract >/dev/null 2>&1; then
    echo "Error: cabextract is not installed. Please install it first."
    echo "On Windows: winget install cabextract"
    echo "On Linux: sudo apt install cabextract"
    echo "On macOS: brew install cabextract"
    exit 1
fi

# Set download URL based on OS type
DL_URL="https://downloads.sourceforge.net/corefonts/arial32.exe"

# Download and extract
mkdir -p collect/share/fonts

curl -L ${DL_URL} -o arial32.exe ||
    { echo "Error: Failed to download arial32.exe"; exit 1; }

cabextract --filter='*.TTF' --directory collect/share/fonts arial32.exe ||
    { echo "Error: Failed to extract arial32.exe"; exit 1; }

# Use build_tar function from common.sh
build_tar
