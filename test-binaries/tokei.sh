#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

test_version "tokei" "tokei [0-9]+\.[0-9]+\.[0-9]+"

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

assert 'echo "${OUTPUT}" | grep -q "Rust" && echo "${OUTPUT}" | grep -q "Total"' \
    "Expected output containing 'Rust' and 'Total'"
