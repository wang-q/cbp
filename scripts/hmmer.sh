#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set configure options based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    SIMD_OPT="--enable-sse"
elif [ "$OS_TYPE" == "macos" ]; then
    SIMD_OPT="--enable-neon"
fi

CC="zig cc -target ${TARGET_ARCH}" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    ${SIMD_OPT} \
    --enable-threads \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# Use build_tar function from common.sh
build_tar
