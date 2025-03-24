#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CXX="zig c++ -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS="-I$HOME/.cbp/include -L$HOME/.cbp/lib -std=c99 -g -Wall -O3" \
    || exit 1

# ldd miniprot

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" miniprot ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
