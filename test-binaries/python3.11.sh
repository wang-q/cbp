#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "python3" "Python 3\.[0-9]+\."

# Create test script
echo "-> Creating test script"
cat > test.py << 'EOF'
import sys
import sqlite3
import ssl
import zlib
import ctypes
import multiprocessing

def test_imports():
    modules = [sys, sqlite3, ssl, zlib, ctypes, multiprocessing]
    for module in modules:
        print(f"Successfully imported {module.__name__}")

def test_unicode():
    print("Unicode test: 你好, 世界!")

def test_multiprocessing():
    with multiprocessing.Pool(2) as pool:
        result = pool.map(str, range(3))
        print(f"Multiprocessing test: {result}")

if __name__ == '__main__':
    test_imports()
    test_unicode()
    test_multiprocessing()
    print("Test PASSED")
EOF

# Run test script
echo "-> Running test script"
OUTPUT=$($(cbp prefix bin)/python3 test.py)

assert 'echo "${OUTPUT}" | grep -q "Successfully imported sys"' "Basic module import failed"
assert 'echo "${OUTPUT}" | grep -q "Successfully imported sqlite3"' "SQLite3 module import failed"
assert 'echo "${OUTPUT}" | grep -q "Successfully imported ssl"' "SSL module import failed"
assert 'echo "${OUTPUT}" | grep -q "Successfully imported zlib"' "Zlib module import failed"
assert 'echo "${OUTPUT}" | grep -q "Successfully imported ctypes"' "CTypes module import failed"
assert 'echo "${OUTPUT}" | grep -q "Successfully imported multiprocessing"' "Multiprocessing module import failed"
assert 'echo "${OUTPUT}" | grep -q "Unicode test: 你好, 世界!"' "Unicode support test failed"
assert 'echo "${OUTPUT}" | grep -q "\[.0., .1., .2.\]"' "Multiprocessing test failed"
assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Python test script failed"
