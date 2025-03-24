#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I${CBP_INCLUDE}" \
LDFLAGS="-L${CBP_LIB}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --without-curses \
    --without-ncursesw \
    --disable-bz2 \
    --disable-lzma \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# Build tar
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cd $TEMP_DIR
cbp collect --shebang -o "${FN_TAR}" collect/ ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
