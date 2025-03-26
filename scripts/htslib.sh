#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
AR="zig ar" \
CFLAGS="-I${CBP_INCLUDE}" \
LDFLAGS="-L${CBP_LIB}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --with-libdeflate \
    --disable-bz2 \
    --disable-gcs \
    --disable-s3 \
    --disable-plugins \
    --disable-lzma \
    --disable-libcurl \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
