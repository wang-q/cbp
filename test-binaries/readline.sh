#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <readline/readline.h>

int main()
{
    printf("%s\n", readline("test> "));
    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libreadline.a" \
        -o test.exe
    echo "Hello, World!" | ./test.exe > output.txt
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libreadline.a" \
        -o test
    echo "Hello, World!" | ./test > output.txt
fi

# Check output
assert 'grep -q "^test> Hello, World!$" output.txt' "Expected readline test to pass"
