#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
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

# ldd ${TEMP_DIR}/collect/bin/pv
# eza -T ${TEMP_DIR}/collect/

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect -o "${FN_TAR}" collect ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
