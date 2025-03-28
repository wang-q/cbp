#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    || exit 1

make || exit 1
make install || exit 1

rm -rf ${TEMP_DIR}/collect/share/doc

# eza -T ${TEMP_DIR}/collect/

PERL_VERSION=$(perl -e '$^V =~ /v(.+)/ and print $1')
mv ${TEMP_DIR}/collect/lib/perl5/site_perl/${PERL_VERSION} \
    ${TEMP_DIR}/collect/lib/stow

# # ldd ${TEMP_DIR}/collect/bin/pv
eza -T ${TEMP_DIR}/collect/

# Fix perl module path
for f in stow chkstow; do
    sed -i'.bak' \
        's#use lib "[^"]*";#use FindBin;\nuse lib "$FindBin::RealBin/../lib/stow";#' \
        "${TEMP_DIR}/collect/bin/$f"
done
rm -f ${TEMP_DIR}/collect/bin/*.bak
# head -n 500 ${TEMP_DIR}/collect/bin/stow | tail -n 100
# head -n 10 ${TEMP_DIR}/collect/lib/stow/Stow.pm

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cd $TEMP_DIR/collect
cbp collect --shebang . -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
