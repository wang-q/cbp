#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

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
assert '[ -f "test.png" ]' "PNG file not created"
