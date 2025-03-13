#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
zig cc -target ${TARGET_ARCH} \
    -O3 \
    -finline-functions \
    -funroll-loops \
    -DUSE_DOUBLE \
    -DOPENMP \
    -I/usr/lib/gcc/x86_64-redhat-linux/4.8.2/include/ \
    -L/usr/lib/gcc/x86_64-redhat-linux/4.8.2/ \
    -Wl,-Bstatic \
    -fopenmp=libgomp \
    -lgomp -lpthread -lm \
    FastTree.c \
    -o FastTree ||
    exit 1

ldd FastTree
# ./FastTree

# Collect binaries and create tarball
collect_bins FastTree
build_tar
