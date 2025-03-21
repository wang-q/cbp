#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing resvg installation"

cd "${TEMP_DIR}"

# Create test SVG file
echo "-> Creating test SVG file"
cat > circle.svg << 'EOF'
<svg xmlns="http://www.w3.org/2000/svg" height="100" width="100" version="1.1">
  <circle cx="50" cy="50" r="40" />
</svg>
EOF

# Test resvg conversion
echo "-> Testing resvg SVG to PNG conversion"
$(cbp prefix bin)/resvg circle.svg test.png
if [ -f "test.png" ]; then
    echo "resvg test PASSED"
else
    echo "resvg test FAILED: PNG file not created"
    exit 1
fi

echo "All tests PASSED"
exit 0
