# cbp

[![Publish](https://github.com/wang-q/cbp/actions/workflows/publish.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Build](https://github.com/wang-q/cbp/actions/workflows/build.yml/badge.svg)](https://github.com/wang-q/cbp/actions)
[![Codecov](https://img.shields.io/codecov/c/github/wang-q/cbp/main.svg)](https://codecov.io/github/wang-q/cbp?branch=main)
[![Lines of code](https://www.aschey.tech/tokei/github/wang-q/cbp)](https://github.com//wang-q/cbp)
[![License](https://img.shields.io/github/license/wang-q/builds)](https://github.com/wang-q/builds/blob/main/LICENSE)

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
cbp.linux init
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

Current release: 0.3.2

```text
`cbp` is a Cross-platform Binary Package manager

Usage: cbp.exe [COMMAND]

Commands:
  init     Initialize cbp environment
  install  Download and install packages from GitHub
  local    Install packages from local binaries
  list     List installed packages and their contents
  remove   Remove installed packages
  avail    List available packages from GitHub
  check    Check for unmanaged files in ~/.cbp
  tar      Create compressed archive
  prefix   Display cbp installation directories
  kb       Display project documentation
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


Package Manager Features:
    * Cross-platform support (Linux/macOS/Windows)
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
1. Initial Setup:
   cbp init                    # default setup
   cbp init /opt/cbp           # custom location

2. Package Installation:
   cbp install zlib            # from GitHub
   cbp local zlib              # from local files
   # Use --proxy for restricted networks
   # cbp install --proxy socks5://127.0.0.1:7890 zlib

3. Package Management:
   cbp list                    # list all packages
   cbp list zlib               # show package contents
   cbp remove zlib             # remove package

4. Package Discovery:
   cbp avail                   # list all packages
   cbp avail macos             # platform specific

5. Development Tools:
   cbp check                   # find unmanaged files
   cbp tar -o pkg.tar.gz src/  # create package
   cbp prefix                  # show install paths

6. Documentation:
   cbp kb readme               # view documentation

```

## Supported Packages

Current supported packages can be viewed using `cbp avail`.

Or visit the [Release page](https://github.com/wang-q/cbp/releases/tag/Binaries).

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
