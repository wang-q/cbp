#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build cff
pushd cimfomfa

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
    ./configure \
    --prefix="${TEMP_DIR}/cff" \
    --disable-silent-rules \
    --disable-dependency-tracking \
    --enable-static \
    --disable-shared \
    || exit 1
make -j 8 || exit 1
make install || exit 1

popd

# Build mcl

# ./configure --help

CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I${TEMP_DIR}/cff/include" \
CPPFLAGS="-I${TEMP_DIR}/cff/include" \
LDFLAGS="-L${TEMP_DIR}/cff/lib" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-silent-rules \
    --disable-dependency-tracking \
    --enable-static \
    --disable-shared \
    --enable-rcl \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# ldd $TEMP_DIR/collect/bin/mcl
# eza -T $TEMP_DIR/collect

# Build tar
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cd $TEMP_DIR
cbp collect --shebang -o "${FN_TAR}" collect/ ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
