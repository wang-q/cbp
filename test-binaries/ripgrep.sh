#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing ripgrep installation"

cd "${TEMP_DIR}"

# Create test files
echo "-> Creating test files"
cat > test.txt << 'EOF'
Hello World
This is a test line
Another test line
Final line
EOF

mkdir -p subdir
cat > subdir/test2.txt << 'EOF'
More test content
No match here
Test line again
EOF

# Test basic search
echo "-> Testing basic search"
RESULT=$($(cbp prefix bin)/rg -i "test" --no-heading --no-line-number --no-filename | sort)
EXPECTED=$'Another test line\nMore test content\nTest line again\nThis is a test line'
EXPECTED=$(echo "$EXPECTED" | tr -d '\r' | sort)
RESULT=$(echo "$RESULT" | tr -d '\r')

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Basic search test PASSED"
else
    echo "Basic search test FAILED"
    echo "Expected:"
    echo "$EXPECTED"
    echo "Got:"
    echo "$RESULT"
    exit 1
fi

echo "All tests PASSED"
exit 0
