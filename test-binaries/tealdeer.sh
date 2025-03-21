#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test tldr command
echo "-> Testing tldr command"
$(cbp prefix bin)/tldr -u
OUTPUT=$($(cbp prefix bin)/tldr cat)

assert 'echo "${OUTPUT}" | grep -q "concatenate"' "Expected 'concatenate' in output"
