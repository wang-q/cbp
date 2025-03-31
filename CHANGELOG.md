# Change Log

## Unreleased - ReleaseDate

## 0.3.11 - 2025-03-31

- Improved Rust implementation
  - Improved path handling in match_files
  - Optimize file renaming logic
- Added new tools and scripts
  - Added package analysis scripts
- Enhanced package validation
  - Updated schema with flexible version format
  - Added `--schema` option and workflow
- Documentation improvements
  - Reorganized binaries.md
  - Updated command descriptions

## 0.3.10 - 2025-03-27

- Reorganized commands
  - Moved `upload` command under `build` subcommands
- Enhanced package validation
  - Added `--schema` option to `build validate` command
  - Added GitHub workflow for package schema validation

## 0.3.9 - 2025-03-27

- Added new commands
  - Added `build source` command for downloading sources
  - Added `build prebuild` command for prebuilt packages
  - Added `build font` command for font packages
  - Added `build test` command for package testing
  - Added `build validate` command for package configuration validation
- Enhanced package management
  - Added package schema validation with JSON Schema
  - Added package type classification
  - Migrated build scripts to package definitions
  - Added more package metadata and test cases
- Improved testing
  - Enhanced binary test workflow with separate standalone and package tests
  - Added more test coverage
- Documentation updates
- Added more packages

## 0.3.8 - 2025-03-24

- Enhanced package management
  - Added `info` command for package metadata display
  - Added `uninstall` as alias for `remove` command
  - Improved `collect` command
  - Added `prebuild.sh` script for unified prebuilt package handling
  - Migrate prebuild scripts to package definitions: `packages/`
- Improved binary testing workflow
- Updated build system
  - Added LLVM toolchain support
- Improved documentation
- Added more packages

## 0.3.7 - 2025-03-21

- Added automated binary testing workflow
- Added Python 3 support with uv package manager
- Added relocatable Perl 5 support
- Added more vcpkg ports and packages
- Added test scripts for various packages
- Improved build system and documentation

## 0.3.6 - 2025-03-13

- Improved path handling
  - `dunce` for Windows path normalization
  - Simplified executable path resolution
- Enhanced build system
  - Added CentOS 7 container support
  - Improved vcpkg script handling
  - Improved rust handling
- Added new packages
- Updated documentations

## 0.3.5 - 2025-03-10

- Refactored build system
  - Added more vcpkg ports support
  - Moved legacy build scripts to doc/attempts/
  - Updated binaries.md documentation with vcpkg instructions
- Updated development environment
  - Enhanced source package management
  - Improved symlink handling
- Added font management support
  - Added font installation instructions
  - Added font package scripts

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
