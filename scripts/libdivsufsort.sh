#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

cmake -LH .

# # Configure CMake with Zig compiler
# ASM="zig cc" \
# CC="zig cc" \
# CXX="zig c++" \
# CFLAGS="-I$HOME/.cbp/include" \
# LDFLAGS="-L$HOME/.cbp/lib" \
# cmake \
#     -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
#     -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
#     -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
#     -DZLIB_INCLUDE_DIR="$HOME/.cbp/include" \
#     -DZLIB_LIBRARY="$HOME/.cbp/lib/libz.a" \
#     -DINSTALL_GTEST=OFF \
#     -Dspoa_build_executable=ON \
#     -Dspoa_build_tests=OFF \
#     -Dspoa_use_simde=ON \
#     -Dspoa_use_simde_openmp=ON \
#     -S . -B build

# # Build the project
# cmake --build build -- -j 8 || exit 1

# # Collect binaries and create tarball
# FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
# cbp collect --mode bin -o "${FN_TAR}" build/bin/spoa ||
#     { echo "==> Error: Failed to create archive"; exit 1; }
# mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
#     { echo "==> Error: Failed to move archive"; exit 1; }
