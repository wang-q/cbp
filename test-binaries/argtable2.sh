#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include <argtable2.h>

int main(int argc, char **argv) {
    struct arg_lit *help = arg_lit0("h", "help", "print this help and exit");
    struct arg_str *name = arg_str1("n", "name", "NAME", "your name");
    struct arg_end *end = arg_end(20);
    void *argtable[] = {help, name, end};

    if (arg_nullcheck(argtable) != 0) {
        printf("Test FAILED: error allocating argtable\n");
        return 1;
    }

    int nerrors = arg_parse(argc, argv, argtable);
    if (nerrors == 0 && !help->count) {
        printf("Hello, %s!\n", name->sval[0]);
        printf("Test PASSED\n");
    }

    arg_freetable(argtable, sizeof(argtable)/sizeof(argtable[0]));
    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libargtable2.a" \
        -o test.exe
    OUTPUT=$(./test.exe -n "argtable2")
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libargtable2.a" \
        -o test
    OUTPUT=$(./test -n "argtable2")
fi

assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Argtable2 command line parsing test failed"
