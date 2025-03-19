#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

autoreconf -fvi

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
    || exit 1
make -j 8 || exit 1
make install || exit 1

ldd "${TEMP_DIR}/collect/bin/snp-sites"
eza -T "${TEMP_DIR}/collect"

# Use build_tar function from common.sh
build_tar
