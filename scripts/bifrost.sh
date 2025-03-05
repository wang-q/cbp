#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# cmake -LAH .
# cmake -LH .

# Patch CMakeLists.txt to remove dynamic libraries
# sed -i '/add_library.*dynamic/d' src/CMakeLists.txt
# sed -i '/set_target_properties.*dynamic/d' src/CMakeLists.txt
# sed -i '/target_include_directories.*dynamic/d' src/CMakeLists.txt
# sed -i '/target_link_libraries.*dynamic/d' src/CMakeLists.txt
# sed -i '/install.*dynamic/d' src/CMakeLists.txt
sed -i 's/target_link_libraries(Bifrost bifrost_dynamic)/target_link_libraries(Bifrost bifrost_static)/' src/CMakeLists.txt

# Configure CMake with Zig compiler
ASM="zig cc" \
CC="zig cc" \
CXX="zig c++" \
AR="zig ar" \
CFLAGS="-I${CBP_INCLUDE}" \
CXXFLAGS="-I${CBP_INCLUDE}" \
LDFLAGS="-L${CBP_LIB}" \
cmake \
    -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
    -DZLIB_INCLUDE_DIR="${CBP_INCLUDE}" \
    -DZLIB_LIBRARY="${CBP_LIB}/libz.a" \
    -DCMAKE_CXX_FLAGS="-Wno-unqualified-std-cast-call" \
    -DCMAKE_BUILD_TYPE=Release \
    -DMAX_KMER_SIZE=128 \
    -S . -B build

# Build the project
cmake --build build -- -j 8 || exit 1

# ldd build/src/Bifrost

# Use build_tar function from common.sh
collect_bins build/src/Bifrost
build_tar
