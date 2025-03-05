# Cross-compiling Windows binaries on Linux

This project uses `zig cc` for cross-compiling Windows binaries on Linux systems. `zig cc` provides
 a modern, efficient way to build Windows executables and libraries without requiring a traditional
MinGW toolchain.

It supports various build systems including:

- configure scripts
- CMake
- Meson

This makes it suitable for compiling a wide range of C/C++ projects. The target architecture is
`x86_64-windows-gnu`, and the resulting binaries are compatible with modern Windows systems.

To avoid conflicts with existing Linux files, after initializing `cbp` with default settings and
installing required tools, use `cbp init ~/win-build` to store the compiled Windows libraries in a
separate location.

This document follows the same structure as `doc/binaries.md` for consistency and easier reference.

## Setup

```bash
cbp init ~/win-build

```

## Core Libraries

```bash
bash scripts/zlib.sh windows
bash scripts/bzip2.sh windows
bash scripts/libdeflate.sh windows
bash scripts/xz.sh windows

cbp local --type windows zlib bzip2 libdeflate xz

```

## Other Libraries


```bash
bash scripts/argtable.sh windows

```

## `Makefile`

```bash
bash scripts/pigz.sh windows
cbp local --type windows -l pigz

```

## `./configure`

```bash
bash scripts/TRF.sh windows
cbp local --type windows -l TRF

# ./stopwatch.h:23:10: fatal error: 'sys/times.h' file not found
# cbp local --type windows argtable
# bash scripts/clustalo.sh windows

```

## `cmake`

```bash
#  error: no matching function for call to 'max'
# bash scripts/bifrost.sh windows

```
