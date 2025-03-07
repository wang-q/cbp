# vcpkg build environment

## Linux/macOS

```bash
vcpkg install --debug \
    --overlay-ports=ports --overlay-triplets=doc/triplets \
    pigz:x64-linux-zig

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

vcpkg install --debug --recurse `
    --overlay-triplets=doc/triplets `
    zlib:x64-windows-zig

vcpkg install --debug zlib:x64-windows-zig

# vcpkg install --debug htslib[deflate]:x64-windows-zig --allow-unsupported

```
