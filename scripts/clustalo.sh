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
RANLIB="zig ranlib" \
CFLAGS="-I${CBP_INCLUDE} -Wno-deprecated-declarations -Wno-date-time" \
CPPFLAGS="-I${CBP_INCLUDE} -Wno-deprecated-declarations -Wno-date-time" \
CXXFLAGS="-I${CBP_INCLUDE} -Wno-deprecated-declarations -Wno-date-time" \
LDFLAGS="-L${CBP_LIB}" \
LIBS="-largtable2" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-shared \
    --enable-static \
    || exit 1

# Remove -bind_at_load flag from libtool files on macOS
if [[ "$OS_TYPE" == "macos" ]]; then
    # rg 'bind_at_load'
    sed -i.bak 's/${wl}-bind_at_load//' libtool
    find . -name ltmain.sh -exec sed -i.bak 's/${wl}-bind_at_load//' {} \;
fi

make || exit 1
make install || exit 1

# ldd ${TEMP_DIR}/collect/bin/clustalo

# Collect binaries and create tarball
# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
