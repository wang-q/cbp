#!/bin/bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

cd "${BASH_DIR}"/..

# Check if the package name is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <PACKAGE_NAME> [OS_TYPE]"
    echo "Supported OS_TYPE: linux, macos, windows"
    echo "Example: $0 zlib linux"
    exit 1
fi

# Get base package name without features
PROJ=$1
BASE_PROJ=$(echo $PROJ | cut -d'[' -f1)

# Set default OS type based on current system
case "$OSTYPE" in
    darwin*)
        DEFAULT_OS="macos"
        ;;
    linux*)
        DEFAULT_OS="linux"
        ;;
    msys*|cygwin*|mingw*)
        DEFAULT_OS="windows"
        ;;
    *)
        echo "Error: Unsupported operating system: $OSTYPE"
        exit 1
        ;;
esac

# Use provided OS type or default
OS_TYPE=${2:-$DEFAULT_OS}

# Validate the OS type
if [[ "$OS_TYPE" != "linux" && "$OS_TYPE" != "macos" && "$OS_TYPE" != "windows" ]]; then
    echo "Unsupported OS_TYPE: $OS_TYPE"
    echo "Supported OS_TYPE: linux, macos, windows"
    exit 1
fi

# Set the triplet based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    TRIPLET="x64-linux-zig"
elif [ "$OS_TYPE" == "macos" ]; then
    TRIPLET="arm64-macos-zig"
elif [ "$OS_TYPE" == "windows" ]; then
    TRIPLET="x64-windows-zig"
fi

# Install the package using vcpkg
vcpkg install --debug \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-install-root="$(cbp prefix cache)" \
    "${PROJ}:${TRIPLET}" || exit 1

# Find the package list file
# Create archive from the package list
LIST_FILE=$(find "$(cbp prefix cache)/vcpkg/info" -name "${BASE_PROJ}_*_${TRIPLET}.list" -type f | head -n 1)

if [ -z "${LIST_FILE}" ]; then
    echo "Error: Package list file not found for ${BASE_PROJ}:${TRIPLET}"
    exit 1
else
    echo "Found package list: ${LIST_FILE}"
fi

# Create archive from the package list
cbp collect "${LIST_FILE}" || exit 1

# Move archive to the binaries directory
mv "${BASE_PROJ}.${OS_TYPE}.tar.gz" binaries/
