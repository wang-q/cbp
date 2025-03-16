# Windows binaries

This document follows the same structure as `doc/binaries.md` for consistency and easier reference.

## Development Environment

### Zig and Basic Tools

```powershell
scoop install zig # 0.14.0

~\.cargo\bin\cbp.exe init --dev

cbp install cmake ninja jq

```

### vcpkg Setup

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
# Default triplet (x64-windows): Dynamic linking with MSVC runtime
vcpkg install --debug zlib

# Static linking with MSVC runtime
vcpkg install --debug zlib:x64-windows-static

# Static linking with MinGW runtime (gcc-style)
vcpkg install --debug zlib:x64-mingw-static
vcpkg install --debug bzip2[tool]:x64-mingw-static

```

## `vcpkg` libraries

Most packages are built with x64-windows-zig triplet, which uses Zig as the C/C++ compiler:

```powershell
vcpkg remove --x-install-root="$( cbp prefix cache )" zlib:x64-windows-zig

.\scripts\vcpkg.ps1 zlib x64-windows-zig libzlib.a=libz.a
.\scripts\vcpkg.ps1 "bzip2[tool]"
.\scripts\vcpkg.ps1 libdeflate
.\scripts\vcpkg.ps1 "liblzma[tools]" x64-mingw-static

cbp local zlib bzip2 libdeflate liblzma

.\scripts\vcpkg.ps1 ncurses
.\scripts\vcpkg.ps1 readline
.\scripts\vcpkg.ps1 readline-win32

# .\scripts\vcpkg.ps1 argtable2
.\scripts\vcpkg.ps1 expat

.\scripts\vcpkg.ps1 gsl
.\scripts\vcpkg.ps1 simde

.\scripts\vcpkg.ps1 libpng
.\scripts\vcpkg.ps1 openjpeg

```

## `vcpkg` utilities

Some packages rely heavily on MSVC-specific features or Windows SDK. For these packages, we use
x64-windows-static-release triplet to ensure successful builds. Since these are command-line tools,
ABI compatibility is not a concern.

```bash
# avoid icu from sqlite3[*]
.\scripts\vcpkg.ps1 "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]" x64-windows-static-release

.\scripts\vcpkg.ps1 "openssl[core,tools]"

.\scripts\vcpkg.ps1 "curl[core,tool,ssl,http2,websockets]" x64-windows-static-release

.\scripts\vcpkg.ps1 pkgconf x64-windows-static-release pkgconf.exe=pkg-config.exe

# .\scripts\vcpkg.ps1 gdal x64-windows-static-release

# !static
.\scripts\vcpkg.ps1 graphviz x64-windows-static-release

# .\scripts\vcpkg.ps1 gnuplot x64-windows-static-release

# Get-Content "ports\python3\patches\001-unify-site-base.patch" |
#     patch -p1 "C:\Users\wangq\vcpkg\ports\python3\vcpkg-port-config.cmake"

# .\scripts\vcpkg.ps1 python3 x64-windows-release

```

## My ports

```powershell
.\scripts\vcpkg.ps1 pigz
# .\scripts\vcpkg.ps1 faops

Get-Command pigz
(Get-Command pigz).Path

.\scripts\vcpkg.ps1 sickle

.\scripts\vcpkg.ps1 multiz

# .\scripts\vcpkg.ps1 aria2

.\scripts\vcpkg.ps1 trf

```
