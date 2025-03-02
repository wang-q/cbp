#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Modify FastK's linking command to use prebuilt libraries
sed -i 's|LIBDEFLATE/libdeflate.a|'"$HOME"'/bin/lib/libdeflate.a|' Makefile
sed -i 's|HTSLIB/libhts.a|'"$HOME"'/bin/lib/libhts.a|' Makefile
sed -i 's|-lpthread $(HTSLIB_static_LIBS)|-lpthread -lz|' Makefile

# Remove unnecessary build targets and dependencies
sed -i '/^deflate.lib:/,/^$/d' Makefile
sed -i '/^libhts.a:/,/^$/d' Makefile
sed -i '/^HTSLIB\/htslib_static.mk:/,/^$/d' Makefile
sed -i '/^include HTSLIB/d' Makefile
sed -i 's/^all: deflate.lib libhts.a/all:/' Makefile

# Build the project with the specified target architecture and flags
CFLAGS="-I$HOME/bin/include" \
CXXFLAGS="-I$HOME/bin/include" \
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I$HOME/bin/include -L$HOME/bin/lib -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing -D_GNU_SOURCE" \
    || exit 1

# Get binary names from Makefile
BINS=$(cat Makefile | grep "^ALL = " | sed 's/^ALL =//')

# ldd FastK

# Create collect directory and copy binaries
mkdir -p ${TEMP_DIR}/collect
cp ${BINS} ${TEMP_DIR}/collect/

# Use build_tar function from common.sh
build_tar
