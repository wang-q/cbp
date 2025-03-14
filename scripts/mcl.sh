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

# Fix shebang lines in all files
find "${TEMP_DIR}/collect" -type f -print0 |
while IFS= read -r -d '' file; do
    fix_shebang "$file"
done

# ldd $TEMP_DIR/collect/bin/mcl
# eza -T $TEMP_DIR/collect

# Use build_tar function from common.sh
build_tar
