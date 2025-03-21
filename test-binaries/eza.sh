#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing eza installation"

cd "${TEMP_DIR}"

# Test basic file listing
echo "-> Testing basic file listing"
touch test.txt
RESULT=$($(cbp prefix bin)/eza | grep "test.txt")
if [ -z "$RESULT" ]; then
    echo "Basic listing test FAILED"
    echo "Expected to find test.txt in output"
    exit 1
fi
echo "Basic listing test PASSED"

# Test git integration
echo "-> Testing git integration"
FLAGS="--long --git --no-permissions --no-filesize --no-user --no-time --color=never"

# Initialize git repo
git init > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza $FLAGS | grep "test.txt" | awk '{print $1}')
if [ "$RESULT" != "-N" ]; then
    echo "Git untracked test FAILED"
    echo "Expected: -N"
    echo "Got: $RESULT"
    exit 1
fi
echo "Git untracked test PASSED"

# Add file to git
git add test.txt > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza $FLAGS | grep "test.txt" | awk '{print $1}')
if [ "$RESULT" != "N-" ]; then
    echo "Git staged test FAILED"
    echo "Expected: N-"
    echo "Got: $RESULT"
    exit 1
fi
echo "Git staged test PASSED"

# Commit file
git -c user.name="Test" -c user.email="test@example.com" commit -m "Initial commit" > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza $FLAGS | grep "test.txt" | awk '{print $1}')
if [ "$RESULT" != "--" ]; then
    echo "Git committed test FAILED"
    echo "Expected: --"
    echo "Got: $RESULT"
    exit 1
fi
echo "Git committed test PASSED"

echo "All tests PASSED"
exit 0
