# cbp

[![Publish](https://github.com/wang-q/cbp/actions/workflows/publish.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Build](https://github.com/wang-q/cbp/actions/workflows/build.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Codecov](https://img.shields.io/codecov/c/github/wang-q/cbp/main.svg)](https://codecov.io/github/wang-q/cbp?branch=main)
[![Lines of code](https://www.aschey.tech/tokei/github/wang-q/cbp)](https://github.com//wang-q/cbp)
[![License](https://img.shields.io/github/license/wang-q/builds)](https://github.com/wang-q/builds/blob/main/LICENSE)

`cbp` is a **C**ross-platform **B**inary **P**ackage manager for bioinformatics tools, focusing on
glibc 2.17 (CentOS 7) compatibility and Apple Silicon support. Pre-built binaries are cross-compiled
with Zig for consistent builds across platforms.

The name `cbp` is inspired by DNA's "constant base pairing" - a fundamental principle in molecular
biology where A always pairs with T, and G always pairs with C. Just as these base pairs maintain
reliable and consistent DNA structure, `cbp` aims to provide consistent and reliable binary packages
across different platforms.

## System Requirements

* Linux (glibc 2.17+), macOS (Apple Silicon), or Windows WSL
* Bash shell
* curl
* jq

## Install

Current release: 0.0.1

```bash
curl -L https://raw.githubusercontent.com/wang-q/cbp/main/scripts/init.sh | bash

```

> ⚠️ Windows user should run in WSL

First, create the target directory and download the installation script:

```bash
# Create bin directory if it doesn't exist
mkdir -p ~/bin

# Download the installation script
curl -LO https://raw.githubusercontent.com/wang-q/cbp/main/install.sh
chmod +x install.sh

# Download and install jq (required for the installation script)
curl -LO https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64
chmod +x jq-linux-amd64
mv jq-linux-amd64 ~/bin/jq
# or sudo apt install jq

# Verify the installation
bash install.sh -h

```

Make sure `~/bin` is in your `$PATH`. Add the following line to your `~/.bashrc` if needed:

```bash
export PATH="$HOME/bin:$PATH"

```

```bash
cargo install --path . --force # --offline

# Concurrent tests may trigger sqlite locking
cargo test -- --test-threads=1

# build under WSL 2
mkdir -p /tmp/cargo
export CARGO_TARGET_DIR=/tmp/cargo
cargo build

```

## `cbp help`

```text
`cbp` is a package manager

Usage: cbp [COMMAND]

Commands:
  kb    Prints docs (knowledge bases)
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Examples

## Project Design

This project is designed like a package manager (similar to Homebrew), with the following features:

1. Standardized build process
    * Download source code from official releases
    * Extract and prepare in temporary directory
    * Cross-compile with Zig
    * Package and distribute as tarballs

2. Cross-platform support
    * Linux: glibc 2.17 (CentOS 7) compatibility
    * macOS: aarch64 (Apple Silicon) native
    * Zig as cross-compiler for consistent builds

3. Unified directory structure
    * `sources/` - Source packages
    * `scripts/` - Build scripts and common functions
    * `binaries/` - Build artifacts for distribution

4. Modular design
    * `common.sh` - Shared build environment and functions
    * `install.sh` - Package installation manager
    * Individual build script for each package

The main focus is on bioinformatics tools, with special attention to glibc 2.17 (CentOS 7)
compatibility.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## FAQ

### Why use Zig for cross-compilation?

Zig provides a consistent cross-compilation experience across different platforms and targets
specific glibc versions, which is essential for compatibility with older Linux distributions like
CentOS 7.

### How does this compare to Conda/Bioconda?

While Conda provides a comprehensive package management system, this project focuses specifically
on:

- Minimal dependencies (no Python required)
- Static linking where possible
- Specific glibc compatibility
- Apple Silicon native support

### Can I use these binaries in a Docker container?

Yes, these binaries are ideal for Docker containers as they have minimal dependencies and will work
on any Linux system with glibc 2.17 or newer.

### How do I request a new package?

Open an issue on GitHub with the package name, source URL, and any specific build requirements.
