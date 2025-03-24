#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
cd src
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS='-O3 -Wall -Wextra -Werror -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE -Wno-misleading-indentation -Wno-implicit-const-int-float-conversion ${VERSION_FLAGS}' \
    || exit 1

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" lastz ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
