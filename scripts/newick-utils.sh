#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# cmake -LAH .

# https://github.com/tjunier/newick_utils/pull/31/files
sed -i.bak '43i readline.c' src/CMakeLists.txt

# Configure CMake with Zig compiler
ASM="zig cc" \
CC="zig cc" \
CXX="zig c++" \
AR="zig ar" \
cmake \
    -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_LIBXML=OFF \
    -DUSE_LUA=OFF \
    -DBUILD_SHARED_LIBS=OFF \
    -DCMAKE_EXE_LINKER_FLAGS="-static" \
    -DCMAKE_C_FLAGS="-fcommon" \
    -DCMAKE_CXX_FLAGS="-fcommon" \
    -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
    -S . -B build

cmake --build build -- -j 8 || exit 1
cmake --install build

rm -fr ${TEMP_DIR}/collect/lib

# Use build_tar function from common.sh
build_tar
