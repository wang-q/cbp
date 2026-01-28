#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# cmake -LH .

# Configure CMake with Zig compiler
ASM="zig cc" \
CC="zig cc" \
CXX="zig c++" \
CFLAGS="-I$HOME/.cbp/include" \
LDFLAGS="-L$HOME/.cbp/lib" \
cmake \
    -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
    -DBUILD_EXAMPLES=ON \
    -DBUILD_DIVSUFSORT64=ON \
    -DBUILD_SHARED_LIBS=OFF \
    -S . -B build

# Build the project
cmake --build build -- -j 8 || exit 1

eza -T "${TEMP_DIR}/collect"

# # Collect binaries and create tarball
# FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
# cbp collect --mode bin -o "${FN_TAR}" build/bin/spoa ||
#     { echo "==> Error: Failed to create archive"; exit 1; }
# mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
#     { echo "==> Error: Failed to move archive"; exit 1; }
