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
    -DZLIB_INCLUDE_DIR="$HOME/.cbp/include" \
    -DZLIB_LIBRARY="$HOME/.cbp/lib/libz.a" \
    -DBUILD_STATIC=ON \
    -DCMAKE_BUILD_TYPE=Release \
    -S . -B build

# Build the project
cmake --build build -- -j 8 || exit 1

# Collect binaries
collect_bins "build/diamond"

# Use build_tar function from common.sh
build_tar
