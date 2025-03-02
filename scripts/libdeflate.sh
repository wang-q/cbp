#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# cmake -LH .

# Configure CMake with compiler settings
if [ "$OS_TYPE" != "macos" ]; then
    ASM="zig cc" \
    CC="zig cc" \
    CXX="zig c++" \
    cmake \
        -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
        -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
        -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
        -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
        -DLIBDEFLATE_BUILD_SHARED_LIB=OFF \
        -S . -B build
else
    cmake \
        -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
        -DLIBDEFLATE_BUILD_SHARED_LIB=OFF \
        -S . -B build
fi

# Build the project
cmake --build build -- -j 8 || exit 1
cmake --install build || exit 1

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        echo "test" > foo
        "${TEMP_DIR}/collect/bin/libdeflate-gzip" foo
        RESULT=$("${TEMP_DIR}/collect/bin/libdeflate-gunzip" -dc foo.gz)
        [ "$RESULT" = "test" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Use build_tar function from common.sh
build_tar
