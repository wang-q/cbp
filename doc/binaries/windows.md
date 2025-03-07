# Windows binaries

This document follows the same structure as `doc/binaries.md` for consistency and easier reference.

## Development Environment

```powershell
scoop install zig # 0.14.0

~\.cargo\bin\cbp.exe init --dev

cbp install cmake ninja jq

```

* vcpkg

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


## `vcpkg` libraries

```powershell
vcpkg remove --x-install-root="$(cbp prefix cache)" zlib:x64-windows-zig

.\scripts\vcpkg.ps1 zlib libzlib.a=libz.a
.\scripts\vcpkg.ps1 bzip2[tool]
.\scripts\vcpkg.ps1 libdeflate
# .\scripts\vcpkg.ps1 liblzma[tools]

cbp local zlib bzip2 libdeflate liblzma

.\scripts\vcpkg.ps1 ncurses
.\scripts\vcpkg.ps1 readline
.\scripts\vcpkg.ps1 readline-win32

# .\scripts\vcpkg.ps1 argtable2
.\scripts\vcpkg.ps1 expat

.\scripts\vcpkg.ps1 gsl

```

## `vcpkg` utilities


```bash
# avoid icu from sqlite3[*]
# .\scripts\vcpkg.ps1 "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]"

.\scripts\vcpkg.ps1 "openssl[core,tools]"

# .\scripts\vcpkg.ps1 "curl[core,tool,ssl,http2,websockets]"

# .\scripts\vcpkg.ps1 pkgconf pkgconf=pkg-config

# .\scripts\vcpkg.ps1 graphviz
# gdal

```
