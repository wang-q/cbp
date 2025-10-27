#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# cmake -S src -LH

# Configure CMake with Zig compiler
ASM="gcc" \
CC="gcc" \
CXX="g++" \
CFLAGS="-I$HOME/.cbp/include -w" \
CXXFLAGS="-I$HOME/.cbp/include -w" \
LDFLAGS="-L$HOME/.cbp/lib" \
cmake \
    -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
    -DSPADES_STATIC_BUILD=ON \
    -DSPADES_USE_MIMALLOC=OFF \
    -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
    -S src -B build

# Build the project
cmake --build build -- -j 16 || exit 1
cmake --install build || exit 1

ldd "${TEMP_DIR}/collect/bin/spades-core"
eza -T "${TEMP_DIR}/collectd/bin"

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
