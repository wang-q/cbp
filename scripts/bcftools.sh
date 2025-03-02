#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I$HOME/.cbp/include" \
LDFLAGS="-L$HOME/.cbp/lib" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-bz2 \
    --disable-lzma \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# Use build_tar function from common.sh
build_tar
