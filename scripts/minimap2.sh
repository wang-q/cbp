#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set make options based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    MAKE_OPT=""
elif [ "$OS_TYPE" == "macos" ]; then
    MAKE_OPT="arm_neon=1 aarch64=1"
fi

make \
    extra \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS="-I$HOME/.cbp/include -L$HOME/.cbp/lib -g -Wall -Wno-unused-function -O2" \
    ${MAKE_OPT} \
    || exit 1

# Collect binaries and create tarball
build_tar
