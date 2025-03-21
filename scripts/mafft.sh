#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Replace paths in Makefile and mafft.tmpl
perl -pi -e 's{\$\(LIBDIR\)/}{../libexec/mafft/}g' core/Makefile
perl -pi -e 's{prefix=_LIBDIR}{prefix="\$( cd "\$( dirname "\${BASH_SOURCE[0]}" )" && pwd )/../libexec/mafft"}' core/mafft.tmpl

# Build the project with the specified target architecture and flags
cd core
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    CFLAGS='-O3 -std=c99 -Wno-deprecated-non-prototype -Wno-parentheses -Wno-fortify-source' \
    PREFIX="../collect" \
    ENABLE_MULTITHREAD= \
    install \
    || exit 1

mkdir -p ${TEMP_DIR}/collect

cp -R ../collect/* ${TEMP_DIR}/collect/

# eza -T ${TEMP_DIR}/collect/
# otool -L ${TEMP_DIR}/collect/libexec/mafft/mafft-distance
# head -n 50 ${TEMP_DIR}/collect/bin/mafft

# Collect binaries and create tarball
build_tar
