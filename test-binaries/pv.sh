#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "pv" "pv" "--version"

# Test basic functionality
echo "-> Testing basic transfer"
dd if=/dev/zero bs=1M count=10 2>/dev/null | \
    $(cbp prefix bin)/pv > /dev/null
assert_eq "$?" "0" "Basic transfer should succeed"
