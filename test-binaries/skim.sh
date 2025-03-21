#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

test_version "sk" "^sk [0-9]+\.[0-9]+\.[0-9]+$"
