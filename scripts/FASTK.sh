#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Modify FastK's linking command to use prebuilt libraries
sed -i.bak 's|LIBDEFLATE/libdeflate.a|'"$HOME"'/.cbp/lib/libdeflate.a|' Makefile
sed -i.bak 's|HTSLIB/libhts.a|'"$HOME"'/.cbp/lib/libhts.a|' Makefile
sed -i.bak 's|-lpthread $(HTSLIB_static_LIBS)|-lpthread -lz|' Makefile

# Remove unnecessary build targets and dependencies
sed -i.bak '/^deflate.lib:/,/^$/d' Makefile
sed -i.bak '/^libhts.a:/,/^$/d' Makefile
sed -i.bak '/^HTSLIB\/htslib_static.mk:/,/^$/d' Makefile
sed -i.bak '/^include HTSLIB/d' Makefile
sed -i.bak 's/^all: deflate.lib libhts.a/all:/' Makefile

# Build the project with the specified target architecture and flags
CFLAGS="-I$HOME/.cbp/include" \
CXXFLAGS="-I$HOME/.cbp/include" \
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I$HOME/.cbp/include -L$HOME/.cbp/lib -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing -D_GNU_SOURCE" \
    || exit 1

# Get binary names from Makefile
BINS=$(cat Makefile | grep "^ALL = " | sed 's/^ALL =//')

# ldd FastK

# Use build_tar function from common.sh
collect_bins ${BINS}
build_tar
