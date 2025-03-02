#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
    ./configure \
    --static \
    --prefix="${TEMP_DIR}/collect"
make
make install

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    source "${BASH_DIR}/tests/zlib.sh"
    create_and_build_test
    run_test "${TEMP_DIR}/test" "zlib"
fi

# Create package
build_tar
