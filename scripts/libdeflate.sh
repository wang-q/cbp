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
cmake \
    -DCMAKE_ASM_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_C_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_CXX_COMPILER_TARGET="${TARGET_ARCH}" \
    -DCMAKE_INSTALL_PREFIX="${TEMP_DIR}/collect" \
    -DLIBDEFLATE_BUILD_SHARED_LIB=OFF \
    -S . -B build

# Build the project
cmake --build build -- -j 8 || exit 1
cmake --install build || exit 1

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    cd "${TEMP_DIR}"
    echo "test" > foo
    "${TEMP_DIR}/collect/bin/libdeflate-gzip" foo
    RESULT=$("${TEMP_DIR}/collect/bin/libdeflate-gunzip" -dc foo.gz)
    if [ "$RESULT" = "test" ]; then
        echo "==> Test PASSED"
    else
        echo "==> Test FAILED"
        echo "Expected: test"
        echo "Got: $RESULT"
        exit 1
    fi
else
    echo "==> Skipping tests (use 'test' as second argument to enable)"
fi

# Use build_tar function from common.sh
build_tar
