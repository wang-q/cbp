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

## Features

* Cross-platform compatibility
  - Linux (glibc 2.17+)
  - macOS (Apple Silicon)
  - Windows WSL
* Minimal dependencies
  - Bash shell
  - curl
* Package management
  - GitHub release integration
  - Local package support
  - Package tracking
  - Proxy support

## Quick Start

```bash
# Install cbp
curl -L https://raw.githubusercontent.com/wang-q/cbp/main/scripts/init.sh | bash

# List available packages
cbp avail

# Install packages
cbp install zlib
cbp install --proxy socks5://127.0.0.1:7890 zlib   # with proxy

# Manage packages
cbp list                    # list installed packages
cbp list zlib              # show package contents
cbp remove zlib            # remove package
```

> ⚠️ Windows user should run in WSL

## `cbp help`

```text
`cbp` is a Cross-platform Binary Package manager

Usage: cbp [COMMAND]

Commands:
  install  Download and install packages from GitHub
  list     List installed packages and their contents
  remove   Remove installed packages
  avail    List available packages from GitHub
  local    Install packages from local binaries
  check    Check for unmanaged files in ~/.cbp
  kb       Display project documentation
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


Package Manager Features:
    * Cross-platform support (macOS/Linux)
    * Pre-built static binaries
    * GitHub release integration
    * Local package support
    * Package tracking

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Common Commands:
1. Package Installation:
   cbp install zlib                                   # from GitHub
   cbp install --proxy socks5://127.0.0.1:7890 zlib   # with proxy
   cbp local zlib                                     # from local files

2. Package Management:
   cbp list                                           # list all packages
   cbp list zlib                                      # show package contents
   cbp remove zlib                                    # remove package

3. Package Discovery:
   cbp avail                                          # list available packages
   cbp check                                          # find unmanaged files

4. Documentation:
   cbp kb readme                                      # view documentation

```

## Architecture

1. Package Management
    * Command-line interface
    * Package status tracking
    * File-based record keeping
    * Network proxy support

2. Build System
    * Zig-based cross-compilation
    * Platform-specific optimizations
    * Static linking preferred
    * Reproducible builds

3. Directory Layout
    * Runtime
      - `~/.cbp/`  - User installation directory
      - `bin/`     - Executable files
      - `cache/`   - Downloaded packages
      - `records/` - Package file lists
    * Development
      - `src/`     - Package manager source
      - `tests/`   - Test suites
      - `scripts/` - Build automation
    * Distribution
      - `sources/` - Upstream packages
      - `binaries/` - Pre-built packages
    * `src/`, `tests/` - Rust source code

4. Core Components
    * Package manager (Rust)
      - Installation and removal
      - File tracking
      - Cache management
    * Build system (Bash)
      - Cross-compilation
      - Package creation
      - Testing framework

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
