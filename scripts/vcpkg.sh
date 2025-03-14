#!/bin/bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

cd "${BASH_DIR}"/..

# Check if the package name is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <PACKAGE_NAME> [OS_TYPE] [COPY_PAIRS...]"
    echo "Supported OS_TYPE: linux, macos, windows"
    echo "Example: $0 zlib linux"
    echo "Example with copy: $0 pkgconf linux pkgconf=pkg-config"
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

# Set the triplet based on OS type
# Use provided triplet or default based on OS
TRIPLET=${2}
if [ -z "$TRIPLET" ]; then
    if [ "$DEFAULT_OS" == "linux" ]; then
        TRIPLET="x64-linux-zig"
    elif [ "$DEFAULT_OS" == "macos" ]; then
        TRIPLET="arm64-macos-zig"
    elif [ "$DEFAULT_OS" == "windows" ]; then
        TRIPLET="x64-windows-zig"
    fi
fi

# Extract OS_TYPE from triplet
if [[ "$TRIPLET" == *"-linux"* ]]; then
    OS_TYPE="linux"
elif [[ "$TRIPLET" == *"-macos"* || "$TRIPLET" == *"-osx"* ]]; then
    OS_TYPE="macos"
elif [[ "$TRIPLET" == *"-windows"* ]]; then
    OS_TYPE="windows"
else
    echo "Error: Unsupported triplet: $TRIPLET"
    echo "Triplet must contain one of: linux, macos/osx, windows"
    exit 1
fi

# Install the package using vcpkg
vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
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

# Process copy arguments
COPY_ARGS=()
shift 2  # Skip package name and OS type
for copy_pair in "$@"; do
    COPY_ARGS+=(--copy "$copy_pair")
done

# Create archive from the package list
cbp collect "${LIST_FILE}" "${COPY_ARGS[@]}" || exit 1

# Rename .osx.tar.gz to .macos.tar.gz if needed
if [ -f "${BASE_PROJ}.osx.tar.gz" ]; then
    mv "${BASE_PROJ}.osx.tar.gz" "${BASE_PROJ}.macos.tar.gz"
fi

# # Remove the package from cache
# vcpkg remove --recurse \
#     --overlay-ports=ports \
#     --overlay-triplets="$(cbp prefix triplets)" \
#     --x-buildtrees-root=vcpkg/buildtrees \
#     --downloads-root=vcpkg/downloads \
#     --x-install-root=vcpkg/installed \
#     --x-packages-root=vcpkg/packages \
#     "${BASE_PROJ}:${TRIPLET}"

# Move archive to the binaries directory
mv "${BASE_PROJ}.${OS_TYPE}.tar.gz" binaries/
