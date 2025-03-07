#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS="-I${CBP_INCLUDE} -Wall -pedantic -DVERSION=1.33" \
    LDFLAGS="-L${CBP_LIB}" \
    || exit 1

# Collect binaries and create tarball
collect_bins sickle
build_tar
