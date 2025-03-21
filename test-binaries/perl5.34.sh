#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "perl" "This is perl.*v5\.[0-9]+"

# Create test script
echo "-> Creating test script"
cat > test.pl << 'EOF'
use strict;
use warnings;
use utf8;
use Compress::Zlib;
use threads;

# Test Unicode support
print "Unicode test: 你好, 世界!\n";

# Test Zlib compression
my $string = "Hello, Perl!";
my $compressed = Compress::Zlib::compress($string);
my $decompressed = Compress::Zlib::uncompress($compressed);
print "Compression test: ", ($string eq $decompressed ? "PASSED" : "FAILED"), "\n";

# Test threading
my $thread = threads->create(sub {
    return "Thread test: PASSED";
});
print $thread->join(), "\n";

print "Test PASSED\n";
EOF

# Run test script
echo "-> Running test script"
OUTPUT=$($(cbp prefix bin)/perl test.pl)

assert 'echo "${OUTPUT}" | grep -q "Unicode test: 你好, 世界!"' "Unicode support test failed"
assert 'echo "${OUTPUT}" | grep -q "Compression test: PASSED"' "Zlib compression test failed"
assert 'echo "${OUTPUT}" | grep -q "Thread test: PASSED"' "Threading test failed"
assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Perl test script failed"
