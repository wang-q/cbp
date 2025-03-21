#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Copy example files
echo "-> Copying example files"
cp -R $(cbp prefix)/paml/examples/mtCDNAape/* .

echo "-> Testing codeml"
OUTPUT=$($(cbp prefix bin)/codeml 2>&1)

assert '[ -f "2NG.dN" ]' "PAML codeml failed to generate output file"
assert 'grep -q "6.Gorilla_Arnason" 2NG.dN' "PAML codeml output does not contain expected content"
