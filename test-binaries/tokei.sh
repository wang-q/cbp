#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing tokei installation"

cd "${TEMP_DIR}"

# Create test Rust file
echo "-> Creating test Rust file"
cat > lib.rs << 'EOF'
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        println!("It works!");
    }
}
EOF

# Test tokei analysis
echo "-> Testing tokei analysis"
OUTPUT=$($(cbp prefix bin)/tokei lib.rs)

if echo "$OUTPUT" | grep -q "Rust" && echo "$OUTPUT" | grep -q "Total"; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected output containing 'Rust' and 'Total'"
    echo "Got: $OUTPUT"
    exit 1
fi
