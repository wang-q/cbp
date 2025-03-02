#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --bindir="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    --disable-shared \
    --enable-static \
    --enable-sigwinch \
    --enable-widec \
    --with-shared=no \
    --with-cxx-shared=no \
    --with-gpm=no \
    --without-ada \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# tree "${TEMP_DIR}/collect"
# ldd "${TEMP_DIR}/collect/clear"

# Use build_tar function from common.sh
build_tar
