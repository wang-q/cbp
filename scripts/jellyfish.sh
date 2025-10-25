#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I${CBP_INCLUDE}" \
LDFLAGS="-L${CBP_LIB}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-silent-rules \
    --disable-dependency-tracking \
    --enable-static \
    --disable-shared \
    --enable-all-static \
    || exit 1
make -j 8 || exit 1
make install || exit 1

#ldd "${TEMP_DIR}/collect/bin/jellyfish"
#eza -T "${TEMP_DIR}/collect"

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
