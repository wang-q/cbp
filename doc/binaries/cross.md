# Cross-compiling Windows binaries on Linux

This project uses `zig cc` for cross-compiling Windows binaries on Linux systems. Zig CC provides a
modern, efficient way to build Windows executables and libraries without requiring a traditional
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

# cbp local zlib bzip2 libdeflate xz

```
