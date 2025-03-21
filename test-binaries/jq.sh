#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test JSON file
echo "-> Creating test JSON"
echo '{"foo":1, "bar":2}' > "test.json"

# Test JSON query
echo "-> Testing JSON query"
RESULT=$($(cbp prefix bin)/jq .bar "test.json")
EXPECTED="2"

assert_eq "${RESULT}" "${EXPECTED}" "Expected correct JSON query result"
