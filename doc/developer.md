# Developer Guide

This guide is intended for developers who want to contribute to the `cbp` project or understand its
internal workings.

<!-- TOC -->
- [Developer Guide](#developer-guide)
  - [Development Environment](#development-environment)
    - [Requirements](#requirements)
    - [Setup Build Environment](#setup-build-environment)
    - [Other tools](#other-tools)
    - [git lfs](#git-lfs)
    - [cbp itself](#cbp-itself)
  - [Project Structure](#project-structure)
  - [Build Process](#build-process)
    - [Building Binary Packages](#building-binary-packages)
  - [Dynamic Library Dependencies](#dynamic-library-dependencies)
  - [Uploading Binaries](#uploading-binaries)
    - [Upload Process](#upload-process)
    - [Download URLs](#download-urls)
  - [Contributing](#contributing)
    - [Development Workflow](#development-workflow)
    - [Adding a New Package](#adding-a-new-package)
    - [Example build script templates:](#example-build-script-templates)
<!-- TOC -->

## Development Environment

### Requirements

* Zig compiler (>= 0.14.0)
* Rust toolchain (stable)
* Git (with LFS support)
* `gh` command (GitHub CLI)
* Python 3 (>= 3.7) and uv
* Build tools
    * cmake
    * ninja
    * jq
    * meson

### Setup Build Environment

* Zig

```bash
# Download and install Zig
mkdir -p $HOME/share
cd $HOME/share

# linux and macOS
# zvm
curl https://raw.githubusercontent.com/tristanisham/zvm/master/install.sh | bash
source $HOME/.bashrc

# need 0.14 for pthread on x86_64-windows-gnu
# https://github.com/ziglang/zig/issues/10989
zvm install 0.13.0
zvm install 0.14.0

# # zigup
# curl -L https://github.com/marler8997/zigup/releases/download/v2025_01_02/zigup-x86_64-linux.tar.gz |
#     tar xz &&
#     mv zigup ~/bin/
# zigup fetch 0.14.0

# Verify Zig target
zig targets | jq .libc

# We use the following targets:
# x86_64-linux-gnu.2.17
# aarch64-macos-none
# x86_64-windows-gnu

```

* Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-zigbuild
cargo install --locked cargo-zigbuild

rustup target list

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

```

* vcpkg

```bash
mkdir -p $HOME/share
cd $HOME/share

# Download and extract vcpkg
curl -L https://github.com/microsoft/vcpkg/archive/refs/tags/2025.02.14.tar.gz |
    tar xvz &&
    mv vcpkg-* vcpkg

cd vcpkg
./bootstrap-vcpkg.sh -disableMetrics

# Set environment variables
export VCPKG_ROOT=$HOME/share/vcpkg
export PATH=$VCPKG_ROOT:$PATH

# List all available features for a package
vcpkg search bzip2

# To remove a vcpkg package
vcpkg install --debug --recurse \
    --clean-buildtrees-after-build --clean-packages-after-build \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    zlib:x64-linux-zig

vcpkg remove --debug --recurse \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    zlib:x64-linux-zig

# Install zlib with custom target
# vcpkg install zlib:x64-linux-zig \
#     --cmake-args="-DCMAKE_C_COMPILER_TARGET=aarch64-macos-none" \
#     --cmake-args="-DCMAKE_CXX_COMPILER_TARGET=aarch64-macos-none"

```

* llvm

```bash
cd ~/share

# Download and install llvm
curl -o llvm.tar.xz -L https://github.com/llvm/llvm-project/releases/download/llvmorg-18.1.8/clang+llvm-18.1.8-arm64-apple-macos11.tar.xz

tar xvf llvm.tar.xz
mv clang+llvm-* llvm

# Remove quarantine attribute if exists (ignore errors)
for d in bin lib libexec; do
    for f in llvm/${d}/*; do
        xattr -d com.apple.quarantine "$f" 2>/dev/null || true
    done
done

llvm/bin/lld --version

```

### Other tools

```bash
cbp install cmake
cbp install ninja
cbp install uv
cbp install jq

uv pip install --system meson

```

### git lfs and gh

```bash
# linux
sudo apt install git-lfs
sudo apt install gh

# macos
brew install git-lfs

git lfs install
git lfs track "sources/*.tar.gz"

```

### cbp itself

```bash
cargo install --path . --force # --offline

cargo test -- --test-threads=1

# build under WSL 2
mkdir -p /tmp/cargo
export CARGO_TARGET_DIR=/tmp/cargo
cargo build

cargo run --release --bin cbp init --dev

git log v0.3.11..HEAD > gitlog.txt
git diff HEAD v0.3.11 > gitdiff.txt

```

## Project Structure

```text
cbp/
|-- binaries/      # Build artifacts
|-- doc/           # Documentation
|-- scripts/       # Build scripts
|   |-- common.sh  # Shared build functions
|   |-- tools/     # Helper scripts
|   |-- *.sh       # Package-specific build scripts
|-- sources/       # Source packages
|-- src/           # Rust source code
|-- tests/         # Rust test code

```

## Build Process

### Building Binary Packages

Binary packages are built using shell scripts in the `scripts/` directory. Each package has its own
build script that sources `common.sh` for shared functionality.

See [Common Shell Library](common.md) for detailed information about the shared build
functions and variables.

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
$ ldd ~/.cbp/bin/trimal
        linux-vdso.so.1 (0x00007ffff4599000)
        libstdc++.so.6 => /lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007f796772c000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f7967643000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f7967615000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f7967403000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f79679b6000)

$ bash scripts/tools/deps.sh trimal
==> Dependencies for package trimal:
  File: bin/readal
    No additional dependencies

  File: bin/statal
    No additional dependencies

  File: bin/trimal
    No additional dependencies

$ bash scripts/tools/deps.sh muscle
==> Dependencies for package muscle:
  File: bin/muscle
    Static executable

$ bash scripts/tools/deps.sh bwa
==> Dependencies for package bwa:
  File: bin/bwa
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007de17629d000)

```

## Uploading Binaries

Binary packages are uploaded to a special "Binaries" release on GitHub, which is independent of
`cbp`'s version releases.

Create the Binaries release on GitHub if it doesn't exist

```bash
gh release view Binaries

gh release create Binaries \
    --title "Binary Packages" \
    --notes "This release contains pre-built binary packages for various platforms." \
    --prerelease
```

### Upload Process

1. Build the binary package using the build script
    ```bash
    bash scripts/zlib.sh linux
    ```

2. The resulting tarball will be placed in `binaries/`
    ```bash
    ls -l binaries/zlib.*.tar.gz
    ```

3. GitHub CLI should be installed and authenticated by the repository owner
    ```bash
    gh auth login

    # Verify authentication status
    gh auth status
    ```

4. Upload to GitHub Release
    ```bash
    # Upload with cbp command (recommended), which:
    # 1. Calculate MD5 hash for each file
    # 2. Upload files to GitHub Release
    # 3. Update release notes with new hashes
    cbp upload binaries/zlib.*.tar.gz

    # Or upload manually with gh command
    # Note: This method does not update MD5 hashes
    gh release upload Binaries binaries/zlib.*.tar.gz --clobber
    ```

### Download URLs

The binary packages will be available at fixed URLs:

```text
https://github.com/wang-q/cbp/releases/download/Binaries/zlib.linux.tar.gz

https://github.com/wang-q/cbp/releases/download/Binaries/zlib.macos.tar.gz
```

## Packages

```bash
cargo run --bin cbp build test --dir ~/.cbp arial

```

## Contributing

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes following the style guide
4. Run tests and ensure they pass
5. Submit a pull request

### Adding a New Package

1. Add source tarball to `sources/`
2. Create build script in `scripts/`
3. Test build on both Linux and macOS
4. Add tests if applicable
5. Update documentation
6. Submit a pull request

### Example build script templates:

* Build from source:

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

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" program

```
