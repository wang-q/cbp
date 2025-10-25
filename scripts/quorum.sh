#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
# Try to patch gzip_stream.hpp to use standard C++ instead of GCC extensions
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I${CBP_INCLUDE}" \
CXXFLAGS="-I${CBP_INCLUDE}" \
LDFLAGS="-L${CBP_LIB}" \
PKG_CONFIG_PATH="${CBP_LIB}/pkgconfig" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-silent-rules \
    --disable-dependency-tracking \
    --enable-static \
    --disable-shared \
    --enable-all-static \
    || exit 1

# Patch gzip_stream.hpp to replace GCC-specific ext/stdio_filebuf.h with standard alternatives
if [ -f "include/gzip_stream.hpp" ]; then
    echo "Patching include/gzip_stream.hpp to remove GCC-specific extensions..."
    
    # Use sed to directly modify the file instead of patch
    cp include/gzip_stream.hpp include/gzip_stream.hpp.bak
    
    # Replace the problematic include
    sed -i 's/#include <ext\/stdio_filebuf\.h>/#include <streambuf>/' include/gzip_stream.hpp
    
    # Replace the typedef with a custom class definition
    sed -i '/typedef __gnu_cxx::stdio_filebuf<_CharT> stdbuf;/c\
  class popen_streambuf : public std::streambuf {\
  public:\
    popen_streambuf(FILE* f) : file_(f) {}\
    FILE* file() { return file_; }\
  protected:\
    virtual int overflow(int c) {\
      if (c != EOF) {\
        return fputc(c, file_) == EOF ? EOF : c;\
      }\
      return EOF;\
    }\
    virtual std::streamsize xsputn(const char* s, std::streamsize n) {\
      return fwrite(s, 1, n, file_);\
    }\
  private:\
    FILE* file_;\
  };' include/gzip_stream.hpp
    
    # Replace stdbuf references with popen_streambuf
    sed -i 's/stdbuf/popen_streambuf/g' include/gzip_stream.hpp
    
    # Fix the constructor call (remove the second parameter)
    sed -i 's/new popen_streambuf(f, std::ios::out)/new popen_streambuf(f)/' include/gzip_stream.hpp
    
    echo "Successfully patched gzip_stream.hpp"
fi
make -j 8 || exit 1
make install || exit 1

# ldd "${TEMP_DIR}/collect/bin/quorum"
# eza -T "${TEMP_DIR}/collect"

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
   { echo "==> Error: Failed to create archive"; exit 1; }
