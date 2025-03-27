#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <libxml/tree.h>

int main()
{
    xmlDocPtr doc = xmlNewDoc(BAD_CAST "1.0");
    xmlNodePtr root_node = xmlNewNode(NULL, BAD_CAST "root");
    xmlDocSetRootElement(doc, root_node);
    xmlFreeDoc(doc);
    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)/libxml2" \
        test.c \
        "$(cbp prefix lib)/libxml2.a" \
        -liconv -lz \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)/libxml2" \
        test.c \
        "$(cbp prefix lib)/libxml2.a" \
        -liconv -lz \
        -o test
    OUTPUT=$(./test)
fi

# Check if program ran successfully
assert '[ $? -eq 0 ]' "Expected XML document creation test to pass"
