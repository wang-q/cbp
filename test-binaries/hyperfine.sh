#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing hyperfine installation"

# Test basic benchmark
echo "-> Testing basic benchmark"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    COMMAND="timeout 1"
else
    COMMAND="sleep 0.3"
fi

OUTPUT=$($(cbp prefix bin)/hyperfine "$COMMAND")

if echo "$OUTPUT" | grep -q "Benchmark 1: $COMMAND"; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected output containing 'Benchmark 1: $COMMAND'"
    echo "Got: $OUTPUT"
    exit 1
fi
