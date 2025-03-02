#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Modify the Makefile to use zig cc and specify the target architecture
sed -i 's/^\t\s*gcc/\t$(CC)/g' Makefile || exit 1
sed -i "1i CC = zig cc -target ${TARGET_ARCH}" Makefile || exit 1

# Build the project
make || exit 1

# Collect binaries and create tarball
collect_make_bins
build_tar
