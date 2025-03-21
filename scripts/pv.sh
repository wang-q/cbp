#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build mummer with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
AR="zig ar" \
RANLIB="zig ranlib" \
CFLAGS="-I${CBP_INCLUDE} -Wno-implicit-function-declaration" \
CPPFLAGS="-I${CBP_INCLUDE} -Wno-implicit-function-declaration" \
LDFLAGS="-L${CBP_LIB}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-silent-rules \
    --disable-shared \
    --enable-static \
    --disable-nls \
    --without-libiconv-prefix \
    --without-libintl-prefix \
    || exit 1

make || exit 1
make install || exit 1

# ldd ${TEMP_DIR}/collect/bin/clustalo
# eza -T ${TEMP_DIR}/collect/

# Use build_tar function from common.sh
build_tar
