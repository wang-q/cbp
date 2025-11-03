# ![cbp logo](logo.svg)

[![Publish](https://github.com/wang-q/cbp/actions/workflows/publish.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Build](https://github.com/wang-q/cbp/actions/workflows/build.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Codecov](https://img.shields.io/codecov/c/github/wang-q/cbp/main.svg)](https://codecov.io/github/wang-q/cbp?branch=main)
[![Lines of code](https://www.aschey.tech/tokei/github/wang-q/cbp)](https://github.com/wang-q/cbp)
[![License](https://img.shields.io/github/license/wang-q/cbp)](https://github.com/wang-q/cbp/blob/main/LICENSE)

`cbp` is a **C**ross-platform **B**inary **P**ackage manager that simplifies the distribution of CLI
tools, with a focus on bioinformatics software. It ensures compatibility with older Linux systems (
glibc 2.17+), Windows (x86_64), and provides native support for Apple Silicon, using Zig for
reliable cross-platform builds.

The name `cbp` draws inspiration from DNA's **c**onstant **b**ase **p**airing - nature's most
precise pairing system where A pairs with T, and G pairs with C. Similarly, `cbp` ensures consistent
binary compatibility across different platforms, making software distribution as reliable as DNA
replication.

## Features

* Cross-platform compatibility
    - Linux (glibc 2.17+)
    - macOS (Apple Silicon)
    - Windows (x86_64)
* Minimal dependencies
    - Only requires a terminal (Bash/PowerShell)
* Package management
    - Pre-built binaries without dependencies
    - GitHub release integration
    - Local package support
    - Customizable installation paths

## Quick Start

* Linux/macOS

```bash
# Install cbp
curl -LO https://github.com/wang-q/cbp/releases/latest/download/cbp.linux
chmod +x cbp.linux
./cbp.linux init
source ~/.bashrc

# List available packages
cbp avail

# Install packages
cbp install fd jq

# Manage packages
cbp list                 # list installed packages
cbp list fd              # show package contents
cbp remove fd            # remove package

```

* Windows

```powershell
# Install cbp
iwr "https://github.com/wang-q/cbp/releases/latest/download/cbp.windows.exe" -OutFile cbp.windows.exe
.\cbp.windows.exe init

# Restart terminal
# The rest is the same as Linux/macOS

```

> ⚠️ Windows WSL is Linux.

## `cbp help`

Current release: 0.3.14

```console
$ cbp help
`cbp` is a Cross-platform Binary Package manager

Usage: cbp [COMMAND]

Commands:
  init     Initialize cbp environment
  install  Download and install packages from GitHub
  local    Install packages from local binaries
  list     List installed packages and their contents
  remove   Remove installed packages
  info     Display package information
  avail    List available packages from GitHub
  check    Check for unmanaged files
  tar      Create compressed archive
  prefix   Display cbp installation directories
  kb       Display project documentation
  build    Build package commands
  collect  Collect and package files into a tar.gz archive
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


Package Manager Features:
    * Linux/macOS/Windows
    * Pre-built binaries without dependencies
    * Customizable installation paths

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Quick Start:
    cbp init                    # Initial setup
    cbp install <package>       # Install package
    cbp list                    # List installed packages
    cbp avail                   # List available packages
    cbp kb readme               # View more examples

```

## Supported Packages

Current supported packages can be viewed using `cbp avail`.

Or visit the [Release page](https://github.com/wang-q/cbp/releases/tag/Binaries).

## Python 3

`cbp` provides a minimal Python installation. To use Python effectively:

```bash
# Install Python 3
cbp install python3.11 uv

# Set up pip
python3 -m ensurepip --upgrade
python3 -m pip install --upgrade pip setuptools wheel

# Available Python installations
uv python list

# Install Python packages
uv pip install --system numpy matplotlib

```

## Architecture

1. Package Management
    * Command-line interface
    * Package status tracking
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
    * Packages
        - `scripts/` - Build automation
        - `sources/` - Upstream packages
        - `binaries/` - Pre-built packages
    * `src/`, `tests/` - Rust source code

4. Core Components
    * Package manager (Rust)
        - Installation and removal
        - File tracking
        - Cache management
    * Build system (Zig cc)
        - Cross-compilation
        - Package creation
        - Testing framework

For detailed information about development and build process, see
the [Developer Guide](doc/developer.md).

## Contributing

We welcome contributions! Here's how you can help:

1. Report issues
2. Request new packages
3. Submit pull requests
4. Improve documentation

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

## Logo

CBP’s logo conveys reliability and innovation through a clear, science‑forward motif. A dynamic DNA double helix reflects our bioinformatics focus and constant pairing, echoing consistent cross‑platform compatibility. A subtle packaging form symbolizes stewardship and delivery of binaries, tying the helix to practical distribution.
