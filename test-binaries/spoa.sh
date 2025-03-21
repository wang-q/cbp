#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "spoa" "^[0-9]+\.[0-9]+\.[0-9]+$"
