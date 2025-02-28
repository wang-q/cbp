# Developer Guide

This guide is intended for developers who want to contribute to the `cbp` project or understand its
internal workings.

## Development Environment

### Requirements

* Zig compiler
* Rust toolchain
* Git
* `file` command
* Python 3

### Setup Development Environment

* Zig

```bash
# Download and install Zig
mkdir -p $HOME/share
cd $HOME/share

# linux
curl -L https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz > zig.tar.xz
tar xvfJ zig.tar.xz
mv zig-linux-x86_64* zig
ln -s $HOME/share/zig/zig $HOME/bin/zig

# macos
curl -L https://ziglang.org/download/0.13.0/zig-macos-aarch64-0.13.0.tar.xz > zig.tar.xz
tar xvfJ zig.tar.xz
mv zig-macos-aarch64* zig
ln -s $HOME/share/zig/zig $HOME/bin/zig

# Verify Zig target
zig targets | jq .libc

```

* Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-zigbuild
cargo install --locked cargo-zigbuild

rustup target list

rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-apple-darwin

```

### Other tools

```bash
# cmake
curl -LO https://github.com/Kitware/CMake/releases/download/v3.31.5/cmake-3.31.5-linux-x86_64.sh
bash cmake-3.31.5-linux-x86_64.sh
mv cmake-3.31.5-linux-x86_64 cmake
ln -s $HOME/share/cmake/bin/cmake $HOME/bin/cmake

# ninja
curl -LO https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip
chmod +x ninja
mv ninja $HOME/bin/
rm ninja-linux.zip

# meson
pip3 install meson

```

### git lfs

```bash
# linux
sudo apt install git-lfs

# macos
brew install git-lfs

git lfs install
git lfs track "sources/*.tar.gz"

```

## Project Structure

```text
cbp/
|-- binaries/      # Build artifacts
|-- doc/           # Documentation
|-- scripts/       # Build scripts
|   |-- common.sh  # Shared build functions
|   |-- *.sh       # Package-specific build scripts
|-- sources/       # Source packages
|-- src/           # Rust source code
|-- tests/         # Rust test code

```

## Build Process

### Building Binary Packages

Binary packages are built using shell scripts in the `scripts/` directory. Each package has its own
build script that sources `common.sh` for shared functionality.

Example build process:
1. Source code is downloaded to `sources/`
2. Build script extracts and compiles the source
3. Binaries are collected and packaged
4. Resulting tarball is placed in `binaries/`

## Dynamic Library Dependencies

The binaries in this project have minimal dynamic library dependencies:

1. Core System Libraries
    * linux-vdso.so.1 - Virtual dynamic shared object
    * libc.so.6 - GNU C Library (glibc)
    * libpthread.so.0 - POSIX threads library
    * libdl.so.2 - Dynamic linking library
    * /lib64/ld-linux-x86-64.so.2 - Dynamic linker/loader

2. C/C++ Runtime Libraries
    * libstdc++.so.6 - GNU Standard C++ Library
    * libm.so.6 - Math library
    * libgcc_s.so.1 - GCC support library

Example of checking dependencies:

```text
$ ldd ~/bin/trimal
        linux-vdso.so.1 (0x00007ffff4599000)
        libstdc++.so.6 => /lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007f796772c000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f7967643000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f7967615000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f7967403000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f79679b6000)

$ bash install.sh --dep trimal
==> Dependencies for package trimal:
  File: readal
    No additional dependencies

  File: statal
    No additional dependencies

  File: trimal
    No additional dependencies

$ bash install.sh --dep muscle
==> Dependencies for package muscle:
  File: muscle
    Static executable

$ bash install.sh --dep bwa
==> Dependencies for package bwa:
  File: bwa
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fb1c7f8c000)

```

## Contributing

### Adding a New Package

1. Add source tarball to `sources/`
2. Create build script in `scripts/`
3. Test build on both Linux and macOS
4. Update documentation if needed

Example build script template:

```bash
#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    || exit 1

# Collect binaries
collect_make_bins

# Create package
build_tar

```
