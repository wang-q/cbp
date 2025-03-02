#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I$HOME/.cbp/include -L$HOME/.cbp/lib -O3 -static -Wall -Wextra -Wno-unused-result -fno-strict-aliasing -fcommon" \
    || exit 1

# Collect binaries and create tarball
collect_make_bins
build_tar
