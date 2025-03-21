#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "tectonic" "tectonic"

# Test basic compilation
echo "-> Testing basic compilation"
cat > test.tex << 'EOF'
\documentclass{article}
\begin{document}
Hello, World!
\end{document}
EOF

assert '$(cbp prefix bin)/tectonic test.tex' "Basic compilation failed"
assert '[ -f "test.pdf" ]' "PDF file not generated"
