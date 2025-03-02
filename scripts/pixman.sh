#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
meson setup build \
    --prefix="${TEMP_DIR}/collect" \
    --libdir="${TEMP_DIR}/collect/lib" \
    --buildtype=release \
    --wrap-mode=nofallback

meson compile -C build --verbose
meson install -C build

# Use build_tar function from common.sh
build_tar
