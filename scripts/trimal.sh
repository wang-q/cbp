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
collect_bins trimal readal statal
build_tar
