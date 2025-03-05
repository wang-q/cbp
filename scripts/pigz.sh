#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
if [ "$OS_TYPE" == "windows" ]; then
    # Modify Makefile to force .o extension for object files
    # Replace '$(CFLAGS) -c' with '$(CFLAGS) -c -o $*.o'
    perl -pi -e 's{\$\(CFLAGS\) -c}{\$(CFLAGS) -c -o \$*.o}g' Makefile
    CC="gcc"
else
    CC="zig cc -target ${TARGET_ARCH}"    # Modify Makefile to force .exe extension for executable files
fi

make pigz \
    -j 8 \
    CC="$CC" \
    CFLAGS="-I${CBP_INCLUDE} -O3 -Wall -Wextra -Wno-unknown-pragmas -Wcast-qual" \
    LDFLAGS="-L${CBP_LIB}" \
    || exit 1

# Collect binaries and create tarball
collect_bins pigz
build_tar
