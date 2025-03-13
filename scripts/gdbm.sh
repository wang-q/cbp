#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-Wno-date-time" \
LDFLAGS="-static" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    --enable-libgdbm-compat \
    --without-readline \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# ldd ${TEMP_DIR}/collect/bin/gdbmtool

# Use build_tar function from common.sh
build_tar
