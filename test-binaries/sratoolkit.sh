#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "fastq-dump" "fastq-dump : [0-9]+\." "--version"
