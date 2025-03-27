#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include "expat.h"

static void XMLCALL my_StartElementHandler(
    void *userdata,
    const XML_Char *name,
    const XML_Char **atts)
{
    printf("tag:%s|", name);
}

static void XMLCALL my_CharacterDataHandler(
    void *userdata,
    const XML_Char *s,
    int len)
{
    printf("data:%.*s|", len, s);
}

int main()
{
    static const char str[] = "<str>Hello, world!</str>";
    int result;

    XML_Parser parser = XML_ParserCreate("utf-8");
    XML_SetElementHandler(parser, my_StartElementHandler, NULL);
    XML_SetCharacterDataHandler(parser, my_CharacterDataHandler);
    result = XML_Parse(parser, str, sizeof(str), 1);
    XML_ParserFree(parser);

    return result;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libexpat.a" \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libexpat.a" \
        -o test
    OUTPUT=$(./test)
fi

assert 'echo "${OUTPUT}" | grep -q "^tag:str|data:Hello, world!|$"' "Expected XML parsing test to pass"
