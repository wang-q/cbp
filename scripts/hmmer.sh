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

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
