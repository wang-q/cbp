#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set make options based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    CFLAGS="-g -Wall -Wc++-compat -std=c99 -msse4 -O3"
elif [ "$OS_TYPE" == "macos" ]; then
    CFLAGS="-g -Wall -Wc++-compat -std=c99 -O3"
fi

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CXX="zig c++ -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS="-I$HOME/.cbp/include -L$HOME/.cbp/lib ${CFLAGS}" \
    || exit 1

# ldd minigraph

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" minigraph ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
