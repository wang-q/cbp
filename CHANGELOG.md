# Change Log

## Unreleased - ReleaseDate

## 0.3.4 - 2025-03-07

- Migrated to vcpkg-based build system
  - Added custom triplets for cross-compilation
  - Added vcpkg overlay ports support
- Added development environment setup
  - Added `--dev` option to `init` command
  - Added compiler shims for cross-compilation
  - Added vcpkg triplet configurations
- Added `collect` command for package management
- Enhanced Windows support
  - Added MSYS2 building environment
  - Improved toolchain configurations
- Reorganized documentation and build scripts
  - Moved legacy scripts to doc/attempts/
  - Added vcpkg usage guides

## 0.3.3 - 2025-03-05

- Improved Windows support
  - Cross compilation with Zig 0.14.0
  - Added PowerShell build scripts
  - Enhanced binary handling for Windows executables
- Added package content management
  - Added `--list` option to view installed packages
  - Added `--type` option to filter package types
  - Added package content preview
- Added new packages
- Reorganized documentation

## 0.3.2 - 2025-03-04

- Added package manager initialization
  - Added `init` command for environment setup
  - Added `prefix` command for installation paths
- Added Windows support
- Improved proxy support
- Enhanced core utilities

## 0.3.1 - 2025-03-02

- Fixed symlink handling in `tar` command
- Added more software packages
- Improved documentation and help messages

## 0.3.0 - 2025-03-02

- Added `tar` command for creating package archives
- Added `upload` command for managing GitHub releases
- Improved package information tracking with MD5 checksums

## 0.2.0 - 2025-03-01

- Added `install` command with proxy support
- Added `avail` command for package discovery
- Added `local` command for local package installation
- Added `list` command for package management
- Added `check` command for unmanaged files detection
- Added `remove` command with resource fork handling
- Added core utilities for platform detection and package management
- Improved documentation and help messages
- Added `scripts/tools/` directory

## 0.1.0 - 2025-02-28

- Initial release
- Added `kb` command for documentation display
- Supported Linux (glibc 2.17+) and macOS (Apple Silicon)
- Added installation script `init.sh`
- Added build script framework with cross-platform support
