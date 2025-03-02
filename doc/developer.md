# Developer Guide

This guide is intended for developers who want to contribute to the `cbp` project or understand its
internal workings.

<!-- TOC -->
* [Developer Guide](#developer-guide)
  * [Development Environment](#development-environment)
    * [Requirements](#requirements)
    * [Setup Build Environment](#setup-build-environment)
    * [Other tools](#other-tools)
    * [git lfs](#git-lfs)
    * [cbp itself](#cbp-itself)
  * [Project Structure](#project-structure)
  * [Build Process](#build-process)
    * [Building Binary Packages](#building-binary-packages)
  * [Dynamic Library Dependencies](#dynamic-library-dependencies)
  * [Uploading Binaries](#uploading-binaries)
    * [Upload Process](#upload-process)
    * [Download URLs](#download-urls)
  * [Contributing](#contributing)
    * [Development Workflow](#development-workflow)
    * [Adding a New Package](#adding-a-new-package)
    * [Example build script templates:](#example-build-script-templates)
<!-- TOC -->

## Development Environment

### Requirements

* Zig compiler (>= 0.13.0)
* Rust toolchain (stable)
* Git (with LFS support)
* `gh` command (GitHub CLI)
* `file` command
* Python 3 (>= 3.7)
* Build tools
    * cmake
    * ninja
    * meson
    * jq

### Setup Build Environment

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
# linux
cbp install cmake
# mac
brew install cmake

# ninja
cbp install ninja

# meson
pip3 install meson

# jq
cbp install jq
# sudo apt install jq
# brew install jq

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

### cbp itself

```bash
cargo install --path . --force # --offline

# Concurrent tests may trigger sqlite locking
cargo test -- --test-threads=1

# build under WSL 2
mkdir -p /tmp/cargo
export CARGO_TARGET_DIR=/tmp/cargo
cargo build

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

1. Build from source:

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

2. Download pre-built binary:

```bash
#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    DL_URL="https://example.com/package-linux-x86_64"
else
    echo "Error: ${PROJ} does not support ${OS_TYPE}"
    exit 1
fi

# Download
echo "==> Downloading ${PROJ}..."
curl -L "${DL_URL}" -o "${PROJ}" ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }

# Collect binaries
collect_bins "${PROJ}"

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        local output=$("collect/bin/${PROJ}" --version)
        echo "${output}"
        [ -n "${output}" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Pack binaries
build_tar

```
