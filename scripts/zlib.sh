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

# Use build_tar function from common.sh
build_tar
