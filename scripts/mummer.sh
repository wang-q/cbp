#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build mummer with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
LDFLAGS="-static" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    || exit 1

# Remove -bind_at_load flag from libtool files on macOS
if [[ "$OS_TYPE" == "macos" ]]; then
    # rg 'bind_at_load'
    sed -i.bak 's/$wl-bind_at_load//' libtool
    find . -name ltmain.sh -exec sed -i.bak 's/$wl-bind_at_load//' {} \;
fi

make -j 8 || exit 1
make install || exit 1

# Fix shebang lines in all files
find "${TEMP_DIR}/collect" -type f -print0 |
while IFS= read -r -d '' file; do
    fix_shebang "$file"
done

# Rename binary
mv ${TEMP_DIR}/collect/bin/annotate ${TEMP_DIR}/collect/bin/annotate-mummer

# ldd ${TEMP_DIR}/collect/mummer

# Use build_tar function from common.sh
build_tar
