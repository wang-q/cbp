# vcpkg build environment

## Linux/macOS

### Install vcpkg

```bash
cd
# Download and extract vcpkg
curl -L https://github.com/microsoft/vcpkg/archive/refs/tags/2025.02.14.tar.gz |
    tar xvz &&
    mv vcpkg-* vcpkg

cd vcpkg
./bootstrap-vcpkg.sh -disableMetrics

# Set environment variables
export VCPKG_ROOT=$HOME/vcpkg
export PATH=$VCPKG_ROOT:$PATH

```

### Install packages

```bash
# vcpkg remove --debug zlib:x64-linux-zig
vcpkg install --debug zlib:x64-linux-zig
vcpkg install --debug libdeflate:x64-linux-zig

vcpkg install --debug expat:x64-linux-zig
vcpkg install --debug argtable2:x64-linux-zig

vcpkg install --debug sqlite3:x64-linux-zig

vcpkg install --debug libpng:x64-linux-zig
vcpkg install --debug pixman:x64-linux-zig
# vcpkg install --debug cairo:x64-linux-zig

# vcpkg install --debug libxcrypt:x64-linux-zig

vcpkg install --debug c-ares:x64-linux-zig

vcpkg install --debug hdf5:x64-linux-zig

# vcpkg install --debug gsl:x64-linux-zig

# Install zlib with custom target
# vcpkg install zlib:x64-linux-zig \
#     --cmake-args="-DCMAKE_C_COMPILER_TARGET=aarch64-macos-none" \
#     --cmake-args="-DCMAKE_CXX_COMPILER_TARGET=aarch64-macos-none"

vcpkg install --debug --overlay-ports=ports bwa:x64-linux-zig

```


```bash
cbp install cmake ninja

brew install autoconf automake autoconf-archive
brew install pkg-config

vcpkg install --debug vcpkg-cmake
vcpkg install --debug vcpkg-cmake-config

# List all available features for a package
vcpkg search htslib

vcpkg install --debug zlib:arm64-macos-zig
vcpkg install --debug bzip2[tool]:arm64-macos-zig
vcpkg install --debug libdeflate:arm64-macos-zig
vcpkg install --debug liblzma[tools]:arm64-macos-zig

vcpkg install --debug pkgconf:arm64-macos-zig

vcpkg install --debug sqlite3[core,tool,fts3,fts4,fts5,math,rtree,json1]:arm64-macos-zig

vcpkg install --debug htslib[deflate]:arm64-macos-zig

cat $VCPKG_ROOT/installed/vcpkg/info/zlib_arm64-macos-zig.list

vcpkg install --debug --overlay-ports=ports bwa:arm64-macos-zig

```

## Windows

```powershell
# Download and extract vcpkg
cd $env:USERPROFILE
iwr -Uri "https://github.com/microsoft/vcpkg/archive/refs/tags/2025.02.14.tar.gz" -OutFile "vcpkg.tar.gz"
tar xf vcpkg.tar.gz
Move-Item -Path "vcpkg-*" -Destination "vcpkg"

cd vcpkg
.\bootstrap-vcpkg.bat -disableMetrics

# Set environment variables
$env:VCPKG_ROOT = "$env:USERPROFILE\vcpkg"
$env:Path += ";$env:VCPKG_ROOT"

```


```powershell
cbp install cmake

vcpkg install --debug vcpkg-cmake
vcpkg install --debug vcpkg-cmake-config

# Default triplet (x64-windows): Dynamic linking with MSVC runtime
vcpkg install --debug zlib

# Static linking with MSVC runtime
vcpkg install --debug zlib:x64-windows-static

# Static linking with MinGW runtime (gcc-style)
vcpkg install --debug zlib:x64-mingw-static
vcpkg install --debug bzip2[tool]:x64-mingw-static

vcpkg install --debug zlib:x64-windows-zig
vcpkg install --debug bzip2[tool]:x64-windows-zig
vcpkg install --debug libdeflate:x64-windows-zig
vcpkg install --debug liblzma[tools]:x64-windows-zig

vcpkg install --debug pkgconf:x64-windows-zig

vcpkg install --debug sqlite3[core,tool,fts3,fts4,fts5,math,rtree,json1]:x64-windows-zig

# vcpkg install --debug htslib[deflate]:x64-windows-zig --allow-unsupported

```
