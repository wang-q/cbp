#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I$HOME/bin/include -L$HOME/bin/lib -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing" \
    || exit 1

# Collect binaries and create tarball
collect_make_bins
build_tar
