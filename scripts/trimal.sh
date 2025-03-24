#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

cd source

# Makefile has errors
CC="zig c++ -target ${TARGET_ARCH}"
FLAGS="-O2 -fno-strict-aliasing -fcommon"

$CC $FLAGS -c alignment.cpp
$CC $FLAGS -c rwAlignment.cpp
$CC $FLAGS -c autAlignment.cpp

# Build with the specified target architecture
make \
    -j 8 \
    CC="$CC" \
    FLAGS="$FLAGS" \
    || exit 1

# ldd trimal

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" trimal readal statal ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
