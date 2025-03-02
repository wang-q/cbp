#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Modify the Makefile to use zig cc and specify the target architecture
sed -i.bak 's/^\t\s*gcc/\t$(CC)/g' Makefile || exit 1
sed -i.bak "1i\\
CC = zig cc -target ${TARGET_ARCH}
" Makefile || exit 1

# Remove specific targets from the Makefile
sed -i.bak '/^quiva2DB:/{N;N;d;}' Makefile || exit 1
sed -i.bak '/^DB2quiva:/{N;N;d;}' Makefile || exit 1
sed -i.bak '/^arrow2DB:/{N;N;d;}' Makefile || exit 1
sed -i.bak '/^DB2arrow:/{N;N;d;}' Makefile || exit 1

sed -i.bak \
    -e 's/quiva2DB//g' \
    -e 's/DB2quiva//g' \
    -e 's/arrow2DB//g' \
    -e 's/DB2arrow//g' \
    Makefile || exit 1

# Build the project
make || exit 1

# Get binary names from Makefile
BINS=$(make -p | grep "^all: " | sed 's/^all: //')

# Create collect directory and copy binaries
collect_bins ${BINS}

# Use build_tar function from common.sh
build_tar
