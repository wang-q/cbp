#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Skip testing in GitHub Actions environment
if [ -n "${GITHUB_ACTIONS}" ]; then
    echo "-> Skip testing in GitHub Actions environment"
    exit 0
fi

# Test version output
test_version "Bifrost" "[0-9]+\." "--version"
