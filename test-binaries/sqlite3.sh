#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing sqlite3 installation"

echo "-> Testing SQLite3 version"
VERSION_OUTPUT=$(echo ".version" | $(cbp prefix bin)/sqlite3)
if ! echo "$VERSION_OUTPUT" | grep -q "SQLite [0-9]\+\.[0-9]\+\.[0-9]\+"; then
    echo "Version test FAILED"
    echo "Expected version string containing 'SQLite X.Y.Z'"
    echo "Got: $VERSION_OUTPUT"
    exit 1
fi
echo "Version test PASSED"

# Create test SQL file
echo "-> Creating test database"
cat > "${TEMP_DIR}/school.sql" << 'EOF'
create table students (name text, age integer);
insert into students (name, age) values ('Bob', 14);
insert into students (name, age) values ('Sue', 12);
insert into students (name, age) values ('Tim', 13);
select name from students order by age asc;
EOF

# Run test query
echo "-> Running test query"
RESULT=$($(cbp prefix bin)/sqlite3 < "${TEMP_DIR}/school.sql" | tr -d '\r')
EXPECTED=$'Sue\nTim\nBob'

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $RESULT"
    exit 1
fi
