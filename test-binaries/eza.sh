#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test basic file listing
echo "-> Testing basic file listing"
touch test.txt
RESULT=$($(cbp prefix bin)/eza | grep "test.txt")
assert '[ -n "${RESULT}" ]' "Expected to find test.txt in output"

# Test git integration
echo "-> Testing git integration"
FLAGS="--long --git --no-permissions --no-filesize --no-user --no-time --color=never"

# Initialize git repo
git init > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza ${FLAGS} | grep "test.txt" | awk '{print $1}')
assert_eq "${RESULT}" "-N" "Expected untracked file status"

# Add file to git
git add test.txt > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza ${FLAGS} | grep "test.txt" | awk '{print $1}')
assert_eq "${RESULT}" "N-" "Expected staged file status"

# Commit file
git -c user.name="Test" -c user.email="test@example.com" commit -m "Initial commit" > /dev/null 2>&1
RESULT=$($(cbp prefix bin)/eza ${FLAGS} | grep "test.txt" | awk '{print $1}')
assert_eq "${RESULT}" "--" "Expected committed file status"
