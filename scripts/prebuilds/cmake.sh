#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://github.com/Kitware/CMake/releases/download/v3.31.6/cmake-3.31.6-linux-x86_64.tar.gz"
elif [ "$OS_TYPE" == "macos" ]; then
    # The content is CMake.app
    DL_URL="https://github.com/Kitware/CMake/releases/download/v3.31.6/cmake-3.31.6-macos-universal.tar.gz"
elif [ "$OS_TYPE" == "windows" ]; then
    DL_URL="https://github.com/Kitware/CMake/releases/download/v3.31.6/cmake-3.31.6-windows-x86_64.zip"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download binary
echo "==> Downloading ${PROJ}..."
if [ "$OS_TYPE" == "windows" ]; then
    curl -L "${DL_URL}" -o "${PROJ}.zip" ||
        { echo "Error: Failed to download ${PROJ}"; exit 1; }
    unzip "${PROJ}.zip"
else
    curl -L "${DL_URL}" -o "${PROJ}.tar.gz" ||
        { echo "Error: Failed to download ${PROJ}"; exit 1; }
    tar xzf "${PROJ}.tar.gz"
fi

# Handle different directory structures
if [ "$OS_TYPE" == "macos" ]; then
    mkdir -p collect/bin collect/libexec
    rm cmake-*/CMake.app/Contents/bin/cmake-gui
    mv cmake-*/CMake.app/Contents/bin collect/
    mv cmake-*/CMake.app/Contents/share collect/
else
    mv cmake-* collect
    rm -fr collect/doc
    rm -fr collect/man
fi

# Pack binaries
build_tar
