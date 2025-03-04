#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    --disable-shared \
    --enable-static \
    --enable-cxx \
    --with-pic \
    --without-readline \
    || exit 1
make -j 8 || exit 1
# if [ "${RUN_TEST}" = "test" ]; then
#     make check || exit 1
# fi
make install || exit 1

# eza "${TEMP_DIR}/collect"

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    source "${BASH_DIR}/tests/gmp.sh"
    create_and_build_test
    run_test "${TEMP_DIR}/test"
fi

# Use build_tar function from common.sh
build_tar
